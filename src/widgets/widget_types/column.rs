use crate::text_ex::TextEx;
use crate::widget_holder::{RenderInfo, UpdateInfo, WidgetHolder};
use crate::*;
use macroquad::prelude::*;
use std::any::Any;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct Column {
    holder: WidgetHolder,
    func: Option<Box<dyn FnMut(&mut Column)>>,
}

impl Column {
    pub fn new(f: impl FnMut(&mut Column) + 'static) -> Self {
        Self {
            holder: WidgetHolder::new(false),
            func: Some(Box::new(f)),
        }
    }
}

impl Widget for Column {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let mut new_rect = info.rect;
        new_rect.y -= 5.0;
        
        Some(
            self.holder
                .render(
                    &new_rect,
                    0.0,
                    info.font,
                    0.0,
                    [info.cam_1, info.cam_2, info.cam_3],
                )
                .into(),
        )
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        self.holder.reset();
        
        if let Some(mut f) = self.func.take() {
            f(self);
            self.func = Some(f);
        }
        
        self.holder.retain();
        
        let rect = self
            .holder
            .update(
                &info.rect,
                0.0,
                false,
                info.hover,
                info.mouse,
                0.0,
                info.font,
                info.mouse_action,
            )
            .1;

        Some(vec2(rect.w, rect.h))
    }
}

/////////////////////////////////////////////
// WIDGETS
/////////////////////////////////////////////

impl Column {
    fn last_widget_holder(&mut self) -> &mut WidgetHolder {
        &mut self.holder
    }

    // Helper to generate a unique WidgetId string for auto-IDs
    fn generate_widget_id(&self, prefix: &str) -> String {
        let mut hasher = DefaultHasher::new();
        prefix.hash(&mut hasher);
        self.holder.widgets.len().hash(&mut hasher); // Use current number of holders for uniqueness
        format!("{}_{}", prefix, hasher.finish())
    }

    // WIDGET TYPES

    pub fn text(&mut self, label: impl ToString) -> &mut Text {
        let id = self.generate_widget_id("Text");
        self.last_widget_holder().text(id.into(), label.to_string())
    }

    pub fn text_ex(
        &mut self,
        label: impl ToString,
        color: Color,
        font_size: u16,
        font: Option<Font>,
    ) -> &mut TextEx {
        let id = self.generate_widget_id("text_ex");
        self.last_widget_holder()
            .text_ex(id.into(), label.to_string(), color, font_size, font)
    }

    pub fn text_colored(&mut self, label: impl ToString, color: Color) -> &mut TextEx {
        let id = self.generate_widget_id("text_ex");
        self.last_widget_holder()
            .text_ex(id.into(), label.to_string(), color, 14, None)
    }

    pub fn button(&mut self, label: impl ToString) -> &mut Button {
        self.last_widget_holder()
            .button(().into(), label.to_string())
    }

    pub fn checkbox(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        default_value: bool,
    ) -> &mut Checkbox {
        self.last_widget_holder()
            .checkbox(id.into(), label.to_string(), default_value)
    }

    pub async fn image(
        &mut self,
        id: impl Into<WidgetId>,
        path: impl ToString,
        size: Option<Vec2>,
    ) -> &mut ImageWidget {
        self.last_widget_holder()
            .image(id.into(), path.to_string(), size)
            .await
    }

    pub fn slider(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        slider_info: SliderInfo,
    ) -> &mut Slider {
        self.last_widget_holder()
            .slider(id.into(), label.to_string(), slider_info)
    }

    pub fn progress_bar(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        progress_info: ProgressInfo,
    ) -> &mut ProgressBar {
        self.last_widget_holder()
            .progress_bar(id.into(), label.to_string(), progress_info)
    }

    pub fn dropdown(
        &mut self,
        id: impl Into<WidgetId>,
        items: Vec<impl ToString>,
        default_value: impl ToString,
    ) -> &mut Dropdown {
        let stringed_items: Vec<_> = items.iter().map(|x| x.to_string()).collect();
        let stringed_value = default_value.to_string();

        if !stringed_items.contains(&stringed_value) {
            println!("{stringed_value} not found in items [Dropdown].")
        }

        self.last_widget_holder()
            .dropdown(id.into(), stringed_items, stringed_value)
    }

    pub fn radio_buttons(
        &mut self,
        id: impl Into<WidgetId>,
        options: Vec<impl ToString>,
        default_value: impl ToString,
    ) -> &mut RadioButtons {
        let stringed_options: Vec<_> = options.iter().map(|x| x.to_string()).collect();
        let stringed_value = default_value.to_string();

        if !stringed_options.contains(&stringed_value) {
            println!("{stringed_value} not found in options [RadioButtons].")
        }

        self.last_widget_holder()
            .radio_buttons(id.into(), stringed_options, stringed_value)
    }

    pub fn separator(&mut self) -> &mut Separator {
        self.last_widget_holder().separator(WidgetId::Auto)
    }

    pub fn tabs(
        &mut self,
        id: impl Into<WidgetId>,
        tabs: Vec<impl ToString>,
        default_tab: usize,
    ) -> &mut TabHolder {
        self.last_widget_holder().tabs(
            id.into(),
            tabs.iter().map(|x| x.to_string()).collect(),
            default_tab,
        )
    }

    pub fn textbox(
        &mut self,
        id: impl Into<WidgetId>,
        default_text: impl ToString,
    ) -> &mut TextBox {
        self.last_widget_holder()
            .textbox(id.into(), default_text.to_string())
    }

    pub fn labeled_textbox(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        default_text: impl ToString,
    ) -> &mut TextBox {
        self.last_widget_holder().labeled_textbox(
            id.into(),
            label.to_string(),
            default_text.to_string(),
        )
    }
}
