use crate::text_ex::TextEx;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::*;
use indexmap::IndexSet;
use macroquad::math::u16;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct RenderInfo<'a> {
    pub rect: Rect,
    pub cam_1: &'a Camera2D,
    pub cam_2: &'a Camera2D,
    pub cam_3: &'a Camera2D,
    pub font: &'a Option<Font>,
    pub win_rect: Rect,
    pub same_line: bool,
}

pub struct UpdateInfo<'a> {
    pub rect: Rect,
    pub mouse_action: &'a mut WidgetAction,
    pub hover: bool,
    pub mouse: Vec2,
    pub font: &'a Option<Font>,
    pub win_rect: Rect,
    pub same_line: bool,
}

#[derive(Clone, Default)]
pub struct GlobalRenderTargets {
	pub target_1: Option<RenderTarget>,
	pub target_2: Option<RenderTarget>,
	pub target_3: Option<RenderTarget>,
}

pub type WidgetIdNum = u64;

pub struct WidgetHolder {
    pub same_line: bool,
    pub(crate) widgets: HashMap<WidgetIdNum, Box<dyn Widget>>,
    pub(crate) frame_ids: IndexSet<WidgetIdNum>
}

impl WidgetHolder {
    pub fn new(same_line: bool) -> Self {
        Self {
            same_line,
            widgets: HashMap::new(),
            frame_ids: IndexSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.frame_ids.clear();
    }

    pub fn retain(&mut self) {
        self.widgets.retain(|k, _| self.frame_ids.contains(k));
    }
    
    pub fn render<'a>(
        &self,
        rect: &Rect,
        scroll_y: f32,
        font: &Option<Font>,
        vertical_offset: f32,
        cameras: [&Camera2D; 3],
    ) -> f32 {
        let [cam_1, cam_2, cam_3] = cameras;
        
        let mut holder_rect = Rect::new(0.0, -scroll_y + vertical_offset, 0.0, 0.0);
        
        for i in self.frame_ids.iter() {
            set_camera(cam_1);
            
            let mut info = RenderInfo {
                rect: holder_rect,
                win_rect: *rect,
                same_line: self.same_line,
                font,
                cam_1,
                cam_2,
                cam_3,
            };
            
            let widget_size = self.widgets.get(i).unwrap().render(&mut info);
            
            if let Some(size) = widget_size {
                if self.same_line {
                    let padding = 5.0;
                    holder_rect.x += size.x + padding;
                    
                    if size.y > holder_rect.h  {
                        holder_rect.h = size.y
                    }
                } else {
                    holder_rect.h += size.y + 5.0;
                    
                    if size.x > holder_rect.w {
                        holder_rect.w = size.x
                    }
                }
            }
        }
        
        holder_rect.h
    }

    pub fn update(
        &mut self,
        rect: &Rect,
        vertical_offset: f32,
        show_titlebar: bool,
        hover: bool,
        mouse: Vec2,
        scroll_y: f32,
        font: &Option<Font>,
        mouse_action: &mut WidgetAction,
    ) -> (WidgetAction, Rect) {
        let title_thickness = match show_titlebar {
            false => 0.0,
            _ => 30.0,
        };
        let mut holder_rect = Rect::new(
            rect.x + 5.0,
            rect.y + 5.0 + title_thickness - scroll_y + vertical_offset,
            0.0,
            0.0,
        );

        for i in self.frame_ids.iter() {
            let mut info = UpdateInfo {
                rect: holder_rect, // by value
                win_rect: *rect,   // also by value
                same_line: self.same_line,
                mouse_action,
                hover,
                mouse,
                font,
            };

            let widget_size = self.widgets.get_mut(i).unwrap().update(&mut info);
            
            if let Some(size) = widget_size {
                if self.same_line {
                    let padding = 5.0;
                    holder_rect.x += size.x + padding;
                    
                    if size.y > holder_rect.h  {
                        holder_rect.h = size.y
                    }
                } else {
                    holder_rect.h += size.y + 5.0;
                    
                    if size.x > holder_rect.w {
                        holder_rect.w = size.x
                    }
                }
            }
        }

        (mouse_action.clone(), holder_rect)
    }
}

////////////////////////////////////////////////////////
// WIDGET CREATION
////////////////////////////////////////////////////////

impl WidgetHolder {
    pub fn text(&mut self, id: WidgetId, label: String) -> &mut Text {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("Text:{unique}"), &self.frame_ids, id, &label);

        if !self.widgets.contains_key(&new_id) {
            let w = Text::new(label.clone());
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut Text = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b
    }

    pub fn text_ex(
        &mut self,
        id: WidgetId,
        label: String,
        color: Color,
        font_size: u16,
        font: Option<Font>,
    ) -> &mut TextEx {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(
            &format!("TextEx:{unique}:{font_size}:{font:?}:{color:?}"),
            &self.frame_ids,
            id,
            &label,
        );

        if !self.widgets.contains_key(&new_id) {
            let w = TextEx::new(label.clone(), color, font_size, font);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut TextEx = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b
    }

    pub fn button(&mut self, id: WidgetId, label: String) -> &mut Button {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("Button:{unique}"), &self.frame_ids, id, &label);

        if !self.widgets.contains_key(&new_id) {
            let w = Button::new(label.clone());
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut Button = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b.value = label;
        b
    }

    pub fn checkbox(&mut self, id: WidgetId, label: String, value: bool) -> &mut Checkbox {
        let new_id = create_widget_id("Checkbox", &self.frame_ids, id, "");

        if !self.widgets.contains_key(&new_id) {
            let w = Checkbox::new(label.clone(), value);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut Checkbox = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b.text = label;
        b
    }

    pub async fn image(
        &mut self,
        id: WidgetId,
        path: String,
        size: Option<Vec2>,
    ) -> &mut ImageWidget {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("Image:{unique}"), &self.frame_ids, id, &path);

        if !self.widgets.contains_key(&new_id) {
            let w = ImageWidget::new(path, size).await;
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        self.widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    pub fn slider(&mut self, id: WidgetId, label: String, slider_info: SliderInfo) -> &mut Slider {
        let new_id = create_widget_id(
            &format!(
                "Slider<{}>",
                match slider_info {
                    SliderInfo::Int { .. } => "Int",
                    SliderInfo::Float { .. } => "Float",
                }
            ),
            &self.frame_ids,
            id,
            &label,
        );

        if !self.widgets.contains_key(&new_id) {
            let w = Slider::new(label.clone(), slider_info);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut Slider = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b.text = label;
        b
    }

    pub fn dropdown(
        &mut self,
        id: WidgetId,
        items: Vec<String>,
        default_value: String,
    ) -> &mut Dropdown {
        let label = items.join("|");
        let new_id = create_widget_id("Button", &self.frame_ids, id, &label);

        if !self.widgets.contains_key(&new_id) {
            let w = Dropdown::new(items, default_value);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut Dropdown = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        // b.value = label;
        b
    }

    pub fn separator(&mut self, id: WidgetId) -> &mut Separator {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("Separator:{unique}"), &self.frame_ids, id, unique);

        if !self.widgets.contains_key(&new_id) {
            let w = Separator::new();
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        let b: &mut Separator = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();

        b
    }

    pub fn progress_bar(
        &mut self,
        id: WidgetId,
        label: String,
        progress_info: ProgressInfo,
    ) -> &mut ProgressBar {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(
            &format!(
                "ProgressBar<{}>:{unique}",
                match progress_info {
                    ProgressInfo::Int { .. } => "Int",
                    ProgressInfo::Float { .. } => "Float",
                }
            ),
            &self.frame_ids,
            id,
            &label,
        );

        if !self.widgets.contains_key(&new_id) {
            let w = ProgressBar::new(label.clone(), progress_info);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut ProgressBar = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b.text = label;
        b
    }

    pub fn tabs(&mut self, id: WidgetId, tabs: Vec<String>, default_tab: usize) -> &mut TabHolder {
        let new_id = create_widget_id("TabHolder", &self.frame_ids, id, &tabs.join("|"));

        if !self.widgets.contains_key(&new_id) {
            let w = TabHolder::new(tabs, default_tab);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut TabHolder = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b
    }

    pub fn textbox(&mut self, id: WidgetId, text: String) -> &mut TextBox {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("TextBox:{unique}"), &self.frame_ids, id, "");

        if !self.widgets.contains_key(&new_id) {
            let w = TextBox::new(text.clone(), None);
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);

        // UPDATE STATE
        let b: &mut TextBox = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b
    }
    
    pub fn labeled_textbox(&mut self, id: WidgetId, label: String, text: String) -> &mut TextBox {
        let unique = &self.frame_ids.len().to_string();
        let new_id = create_widget_id(&format!("LabeledTextBox:{unique}"), &self.frame_ids, id, "");
        
        if !self.widgets.contains_key(&new_id) {
            let w = TextBox::new(text.clone(), Some(label));
            self.widgets.insert(new_id, Box::new(w));
        }
        self.frame_ids.insert(new_id);
        
        // UPDATE STATE
        let b: &mut TextBox = self
            .widgets
            .get_mut(&new_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        b
    }
}

pub fn create_widget_id(
    widget_type: &str,
    frame_ids: &IndexSet<WidgetIdNum>,
    id: WidgetId,
    label: &str,
) -> WidgetIdNum {
    // Generate a hash based on a widget type + label + explicit/auto ID
    let mut hasher = DefaultHasher::new();
    widget_type.hash(&mut hasher);

    match id {
        WidgetId::Auto => label.hash(&mut hasher),
        WidgetId::Explicit(s) => {
            s.hash(&mut hasher);
            label.hash(&mut hasher);
        }
    };

    let hash = hasher.finish();

    if frame_ids.contains(&hash) {
        panic!("Widget with this type/label already exists. Please give a unique explicit ID.");
    }

    hash
}
