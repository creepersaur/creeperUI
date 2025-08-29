use crate::text_ex::TextEx;
use crate::ui::mouse_action::{MouseAction, WidgetAction};
use crate::ui::windows::win_resize_handles::WindowResizeHandles;
use crate::ui::windows::window_info::WindowInfo;
use crate::widget_holder::WidgetHolder;
use crate::widgets::*;
use crate::{ActionType, WindowId, WindowProperties, WindowTheme};
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;

pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub rect: Rect,
    pub widget_holder: WidgetHolder,
    pub info: WindowInfo,
    pub taken: bool,
    theme: WindowTheme,
    resize_handles: WindowResizeHandles,

    mouse: Vec2,
    pub open: bool,
    pub active: bool,
    pub hover: bool,
    pub mouse_action: WidgetAction,
    pub dragging: Option<Vec2>,
    pub resizing: bool,
}

impl Window {
    pub fn new(id: WindowId, theme: WindowTheme) -> Window {
        Window {
            theme,
            id,
            taken: false,
            title: "Window".into(),
            rect: Rect::new(0.0, 0.0, 200.0, 150.0),
            info: WindowInfo::new(),
            resize_handles: WindowResizeHandles::new(),
            widget_holder: WidgetHolder::new(),

            open: true,
            mouse: mouse_position().into(),
            active: true,
            hover: false,
            dragging: None,
            resizing: false,
            mouse_action: WidgetAction::new(),
        }
    }

    pub fn set_title(&mut self, title: impl ToString) -> &mut Window {
        self.title = title.to_string();
        self
    }

    pub fn set_titlebar(&mut self, showing: bool) -> &mut Window {
        self.info.show_titlebar = showing;
        self
    }

    pub fn set_pos(
        &mut self,
        position: Vec2,
        action_type: impl Into<ActionType> + Clone,
    ) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.x = position.x;
            self.rect.y = position.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.x = position.x;
            self.rect.y = position.y;
        }

        self
    }

    pub fn set_size(
        &mut self,
        size: Vec2,
        action_type: impl Into<ActionType> + Clone,
    ) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.w = size.x;
            self.rect.h = size.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.w = size.x;
            self.rect.h = size.y;
        }

        self
    }

    pub fn set_properties(&mut self, properties: WindowProperties) -> &mut Window {
        self.info.draggable = properties.draggable;
        self.info.resizable = properties.resizable;
        self.info.closable = properties.closable;
        if let Some(title) = properties.title {
            self.title = title;
        }
        if let Some(position) = properties.position {
            self.set_pos(position, ActionType::EachFrame);
        }
        if let Some(size) = properties.size {
            self.set_size(size, ActionType::EachFrame);
        }

        self
    }

    pub fn set_draggable(&mut self, draggable: bool) -> &mut Window {
        self.info.draggable = draggable;
        self
    }

    pub fn set_resizable(&mut self, resizable: bool) -> &mut Window {
        self.info.resizable = resizable;
        self
    }

    pub fn set_closable(&mut self, closable: bool) -> &mut Window {
        self.info.closable = closable;
        self
    }

    pub fn set_active(&mut self, active: bool) -> &mut Window {
        self.active = active;
        self
    }

    pub fn close(&mut self) -> &mut Window {
        self.open = false;
        self
    }

    pub fn show(&mut self) {
        self.open = true;
    }

    pub fn once(&mut self, f: impl FnOnce(&mut Window)) -> &mut Window {
        if !self.info.ran_once {
            f(self);
        }
        self
    }

    pub fn set_min_size(&mut self, size: Vec2) -> &mut Window {
        self.info.min_size = size;
        self
    }
}

/////////////////////////////////////
// RENDER
/////////////////////////////////////

impl Window {
    pub fn render(&self) {
        // BASE
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.theme.background,
        );

        // TITLEBAR
        if self.info.show_titlebar {
            self.draw_titlebar();
        }

        // OUTLINE
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            2.0,
            match (self.active, self.hover) {
                (false, true) => self.theme.hover_stroke,
                (true, _) => self.theme.active_stroke,
                _ => self.theme.win_stroke,
            },
        );

        self.draw_resize_handles();

        // DRAW TOP-LAYER OF WIDGETS
        let target_3 =
            self.widget_holder
                .render(&self.rect, self.info.show_titlebar, &self.theme.font);

        draw_texture_ex(
            &target_3.texture,
            self.rect.x + 5.0,
            self.rect.y + 5.0 + self.theme.title_thickness,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );
    }

    pub fn draw_titlebar(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.theme.title_thickness,
            match self.active {
                true => self.theme.active_titlebar,
                _ => self.theme.inactive_titlebar,
            },
        );

        draw_text_ex(
            &self.title,
            self.rect.x + 10.0,
            self.rect.y + self.theme.title_thickness - 10.0,
            TextParams {
                font: match &self.theme.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                font_size: 13,
                ..Default::default()
            },
        );

        if self.info.closable {
            self.draw_close_button();
        }
    }

    pub fn draw_close_button(&self) {
        draw_rectangle(
            self.rect.x + self.rect.w - self.theme.title_thickness,
            self.rect.y,
            self.theme.title_thickness,
            self.theme.title_thickness,
            self.info.close_color,
        );

        draw_text_ex(
            "x",
            self.rect.x + self.rect.w - self.theme.title_thickness
                + self.theme.title_thickness / 2.0
                - 5.0,
            self.rect.y + self.theme.title_thickness - 10.0,
            TextParams {
                font: match &self.theme.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                font_size: 14,
                ..Default::default()
            },
        );
    }

    pub fn draw_resize_handles(&self) {
        self.resize_handles.render(&self.rect, &self.theme);
    }
}

/////////////////////////////////////
// UPDATE
/////////////////////////////////////

impl Window {
    pub fn update(&mut self, active_taken: bool, mouse_action: MouseAction) {
        if active_taken {
            self.active = false;
        }
        self.mouse = mouse_position().into();

        let hover = self.rect.contains(self.mouse);
        let window_action = mouse_action == MouseAction::WindowHover(self.id)
            || mouse_action == MouseAction::Normal;
        self.hover = window_action && hover;

        if is_mouse_button_pressed(Left) && window_action {
            if !active_taken {
                self.active = hover;
            } else {
                self.active = false;
            }
        }

        self.taken = false;
        if window_action {
            let widget_action = self.widget_holder.update(
                &self.rect,
                self.info.show_titlebar,
                hover,
                self.mouse,
                &self.theme.font,
            );
            if widget_action.taken {
                self.taken = true;
            }
        }

        if self.info.resizable {
            self.update_resize_handles(window_action, self.taken);
        }
        if self.resizing {
            self.taken = true;
        } else if self.info.closable {
            self.handle_close_button(window_action);
        }
        if self.resize_handles.hovering_handle.is_some() {
            self.taken = true;
        }

        if self.info.draggable {
            self.handle_dragging();
        }

        if let Some(start_offset) = self.dragging {
            self.set_pos(self.mouse + start_offset, ActionType::EachFrame);
            self.clamp();
        } else if self.resizing {
            self.clamp();
        }

        self.widget_holder.ensure_render_targets(
            &self.rect,
            match self.info.show_titlebar {
                false => 0.0,
                _ => self.theme.title_thickness,
            },
        );
    }

    fn handle_dragging(&mut self) {
        let mut title_rect = self.rect.clone();
        title_rect.h = self.theme.title_thickness;

        if self.active
            && !self.resizing
            && !self.info.close_button_pressed
            && is_mouse_button_pressed(Left)
            && title_rect.contains(self.mouse)
        {
            self.dragging = Some(vec2(self.rect.x, self.rect.y) - self.mouse);
        }

        if is_mouse_button_released(Left) {
            self.dragging = None;
        }
    }

    pub fn clamp(&mut self) {
        // CLAMP SIZE

        if self.rect.h < self.theme.title_thickness {
            self.rect.h = self.theme.title_thickness;
        }

        let title_dim = measure_text(
            &self.title,
            match &self.theme.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );
        if self.info.show_titlebar {
            if self.rect.w < title_dim.width.max(self.info.close_button_rect.w - 10.0) + 20.0 {
                self.rect.w = title_dim.width.max(self.info.close_button_rect.w - 10.0) + 20.0
            }
        } else {
            self.rect.w = self.rect.w.max(self.theme.title_thickness);
            self.rect.h = self.rect.h.max(self.theme.title_thickness);
        }

        if self.rect.w < self.info.min_size.x {
            self.rect.w = self.info.min_size.x;
        }

        if self.rect.h < self.info.min_size.y {
            self.rect.h = self.info.min_size.y;
        }

        // CLAMP POSITION

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        } else if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }

        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        } else if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    fn handle_close_button(&mut self, window_action: bool) {
        if !self.info.show_titlebar {
            self.info.close_button_hovered = false;
            return;
        }

        self.info.close_button_rect.x = self.rect.x + self.rect.w - self.theme.title_thickness;
        self.info.close_button_rect.y = self.rect.y;
        self.info.close_button_rect.w = self.theme.title_thickness;
        self.info.close_button_rect.h = self.theme.title_thickness;

        if self.info.close_button_rect.contains(self.mouse) && window_action {
            self.info.close_button_hovered = true;
            if is_mouse_button_pressed(Left) && self.active {
                self.info.close_button_pressed = true;
            } else if is_mouse_button_released(Left) {
                if self.info.close_button_pressed {
                    self.close();
                }
            }
        } else {
            self.info.close_button_hovered = false;
        }

        if is_mouse_button_released(Left) {
            self.info.close_button_pressed = false;
        }

        self.info.close_color = Color::from_vec(self.info.close_color.to_vec().lerp(
            match (
                self.info.close_button_pressed,
                self.info.close_button_hovered,
            ) {
                (false, true) => self.theme.close_button_hover.to_vec(),
                (true, _) => self.theme.close_button_press.to_vec(),
                _ => self.theme.close_button.to_vec(),
            },
            0.2,
        ));
    }

    fn update_resize_handles(&mut self, window_action: bool, taken: bool) {
        self.resize_handles
            .update(&mut self.rect, window_action, taken);
        self.resizing = self.resize_handles.resizing.is_some();
    }
}

/////////////////////////////////////
// WIDGET
/////////////////////////////////////

impl Window {
    pub fn begin_widgets(&mut self) {
        self.widget_holder.reset();
    }

    pub fn end_widgets(&mut self) {
        self.widget_holder.retain();
    }

    pub fn scope(&mut self, mut f: impl FnMut(&mut Window)) -> &mut Window {
        f(self);
        self
    }

    #[must_use = "Async scopes won't run unless you `.await` them."]
    pub async fn scope_async(&mut self, mut f: impl AsyncFnMut(&mut Window)) -> &mut Window {
        f(self).await;
        self
    }

    pub fn scope_if(
        &mut self,
        condition: impl Into<bool>,
        mut f: impl FnMut(&mut Window),
    ) -> &mut Window {
        if condition.into() {
            f(self)
        }
        self
    }

    #[must_use = "Async scopes won't run unless you `.await` them."]
    pub async fn scope_async_if(
        &mut self,
        condition: impl Into<bool>,
        mut f: impl AsyncFnMut(&mut Window),
    ) -> &mut Window {
        if condition.into() {
            f(self).await;
        }
        self
    }

    // WIDGET TYPES

    pub fn text(&mut self, label: impl ToString) -> &mut Text {
        self.widget_holder.text(().into(), label.to_string())
    }

    pub fn text_ex(
        &mut self,
        label: impl ToString,
        color: Color,
        font_size: u16,
        font: Option<Font>,
    ) -> &mut TextEx {
        self.widget_holder
            .text_ex(().into(), label.to_string(), color, font_size, font)
    }

    pub fn button(&mut self, id: impl Into<WidgetId>, label: impl ToString) -> &mut Button {
        self.widget_holder.button(id.into(), label.to_string())
    }

    pub fn checkbox(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        default_value: bool,
    ) -> &mut Checkbox {
        self.widget_holder
            .checkbox(id.into(), label.to_string(), default_value)
    }

    #[must_use = "Images won't load unless you `.await` them."]
    pub async fn image(
        &mut self,
        id: impl Into<WidgetId>,
        path: impl ToString,
        size: Option<Vec2>,
    ) -> &mut ImageWidget {
        self.widget_holder
            .image(id.into(), path.to_string(), size)
            .await
    }

    pub fn slider(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        slider_info: SliderInfo,
    ) -> &mut Slider {
        self.widget_holder
            .slider(id.into(), label.to_string(), slider_info)
    }

    pub fn progress_bar(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        progress_info: ProgressInfo,
    ) -> &mut ProgressBar {
        self.widget_holder
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

        self.widget_holder
            .dropdown(id.into(), stringed_items, stringed_value)
    }

    pub fn separator(&mut self) -> &mut Separator {
        self.widget_holder.separator(WidgetId::Auto)
    }

    pub fn tabs(
        &mut self,
        id: impl Into<WidgetId>,
        tabs: Vec<impl ToString>,
        default_tab: usize,
    ) -> &mut TabHolder {
        self.widget_holder.tabs(
            id.into(),
            tabs.iter().map(|x| x.to_string()).collect(),
            default_tab,
        )
    }
}
