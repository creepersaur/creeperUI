use crate::text_ex::TextEx;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::*;
use indexmap::IndexSet;
use macroquad::math::u16;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct UpdateInfo<'a> {
    pub rect: Rect,
    pub mouse_action: &'a mut WidgetAction,
    pub hover: bool,
    pub mouse: Vec2,
    pub font: &'a Option<Font>,
    pub win_rect: Rect,
}

pub struct RenderInfo<'a> {
    pub rect: Rect,
    pub cam_1: &'a Camera2D,
    pub cam_2: &'a Camera2D,
    pub cam_3: &'a Camera2D,
    pub font: &'a Option<Font>,
    pub win_rect: Rect,
}

pub type WidgetIdNum = u64;

pub struct WidgetHolder {
    pub(crate) widgets: HashMap<WidgetIdNum, Box<dyn Widget>>,
    pub(crate) frame_ids: IndexSet<WidgetIdNum>,

    target_1: Option<RenderTarget>,
    target_2: Option<RenderTarget>,
    target_3: Option<RenderTarget>,
}

impl WidgetHolder {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            frame_ids: IndexSet::new(),

            target_1: None,
            target_2: None,
            target_3: None,
        }
    }

    pub fn reset(&mut self) {
        self.frame_ids.clear();
    }

    pub fn retain(&mut self) {
        self.widgets.retain(|k, _| self.frame_ids.contains(k));
    }

    pub fn ensure_render_targets(&mut self, rect: &Rect, title_thickness: f32) {
        let w = rect.w as u32;
        let h = (rect.h - title_thickness) as u32;

        if self
            .target_1
            .as_ref()
            .map(|t| t.texture.width() as u32 != w)
            .unwrap_or(true)
            || self
                .target_1
                .as_ref()
                .map(|t| t.texture.height() as u32 != h)
                .unwrap_or(true)
        {
            self.target_1 = Some(render_target(w, h));
            self.target_2 = Some(render_target(w, h));
        }

        let sw = screen_width() as u32;
        let sh = screen_height() as u32;
        if self
            .target_3
            .as_ref()
            .map(|t| t.texture.width() as u32 != sw)
            .unwrap_or(true)
            || self
                .target_3
                .as_ref()
                .map(|t| t.texture.height() as u32 != sh)
                .unwrap_or(true)
        {
            self.target_3 = Some(render_target(sw, sh));
        }

        self.target_1
            .clone()
            .unwrap()
            .texture
            .set_filter(FilterMode::Nearest);
        self.target_2
            .clone()
            .unwrap()
            .texture
            .set_filter(FilterMode::Nearest);
        self.target_3
            .clone()
            .unwrap()
            .texture
            .set_filter(FilterMode::Nearest);
    }

    pub fn render(&self, rect: &Rect, show_titlebar: bool, font: &Option<Font>) -> &RenderTarget {
        let scale = 0.01;

        let title_thickness = match show_titlebar {
            false => 0.0,
            _ => 30.0,
        };
        let (zoom_x, zoom_y) = (
            scale / rect.w * 200.0,
            scale / (rect.h - title_thickness) * 200.0,
        );

        let target_1 = self.target_1.as_ref().unwrap();
        let target_2 = self.target_2.as_ref().unwrap();
        let target_3 = self.target_3.as_ref().unwrap();

        let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 5.0 + title_thickness, 0.0, 0.0);
        let cam_1 = &Camera2D {
            zoom: vec2(zoom_x, zoom_y),
            target: vec2(1.0 / zoom_x, 1.0 / zoom_y),
            render_target: Some(target_1.clone()),
            ..Default::default()
        };
        let cam_2 = &Camera2D {
            zoom: vec2(zoom_x, zoom_y),
            target: vec2(1.0 / zoom_x, 1.0 / zoom_y),
            render_target: Some(target_2.clone()),
            ..Default::default()
        };
        let cam_3 = &Camera2D {
            zoom: vec2(
                scale / screen_width() * 200.0,
                scale / screen_height() * 200.0,
            ),
            target: vec2(
                1.0 / scale * screen_width() / 200.0,
                1.0 / scale * screen_height() / 200.0,
            ),
            render_target: Some(target_3.clone()),
            ..Default::default()
        };

        set_camera(cam_3);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        set_camera(cam_1);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        for i in self.frame_ids.iter() {
            let mut info = RenderInfo {
                rect: holder_rect,
                font,
                cam_1,
                cam_2,
                cam_3,
                win_rect: *rect,
            };

            let widget_size = self.widgets.get(i).unwrap().render(&mut info);

            if let Some(size) = widget_size {
                holder_rect.h += size.y + 5.0;
                if holder_rect.w < size.x {
                    holder_rect.w = size.x
                }
            }

            set_camera(cam_1);
        }

        set_default_camera();
        draw_texture_ex(
            &target_1.texture,
            rect.x + 5.0,
            rect.y + 5.0 + title_thickness,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    (rect.w - 5.0).max(0.0),
                    (rect.h - 5.0 - title_thickness).max(0.0),
                )),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &target_2.texture,
            rect.x + 5.0,
            rect.y + 5.0 + title_thickness,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    (rect.w - 5.0).max(0.0),
                    (rect.h - 5.0 - title_thickness).max(0.0),
                )),
                ..Default::default()
            },
        );

        target_3
    }

    pub fn update(
        &mut self,
        rect: &Rect,
        show_titlebar: bool,
        hover: bool,
        mouse: Vec2,
        font: &Option<Font>,
    ) -> WidgetAction {
        let title_thickness = match show_titlebar {
            false => 0.0,
            _ => 30.0,
        };
        let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 5.0 + title_thickness, 0.0, 0.0);
        let mut mouse_action = WidgetAction::new();

        for i in self.frame_ids.iter() {
            let mut info = UpdateInfo {
                rect: holder_rect, // by value
                mouse_action: &mut mouse_action,
                hover,
                mouse,
                font,
                win_rect: *rect, // also by value
            };

            if let Some(size) = self.widgets.get_mut(i).unwrap().update(&mut info) {
                holder_rect.h += size.y + 5.0;
                if holder_rect.w < size.x {
                    holder_rect.w = size.x
                }
            }
        }

        mouse_action
    }
}

////////////////////////////////////////////////////////
// WIDGET CREATION
////////////////////////////////////////////////////////

impl WidgetHolder {
    pub fn text(&mut self, id: WidgetId, label: String) -> &mut Text {
        let new_id = create_widget_id("Text", &self.frame_ids, id, &label);

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
        let new_id = create_widget_id("TextEx", &self.frame_ids, id, &label);

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
        let new_id = create_widget_id("Button", &self.frame_ids, id, &label);

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
        let new_id = create_widget_id("Image", &self.frame_ids, id, &path);

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
        let new_id = create_widget_id(
            &format!(
                "ProgressBar<{}>",
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
}

fn create_widget_id(
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
