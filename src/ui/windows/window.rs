use crate::ui::windows::win_resize_handles::WindowResizeHandles;
use crate::ui::windows::window_info::WindowInfo;
use crate::ui::windows::window_theme::WindowTheme;
use crate::ui::mouse_action::MouseAction;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::windows::action_type::ActionType;
use crate::widgets::widget_holder::WidgetHolder;
use crate::widgets::*;

pub struct Window {
    pub id: String,
    pub title: String,
    pub rect: Rect,
    pub widget_holder: WidgetHolder,
    pub info: WindowInfo,
    theme: WindowTheme,
    resize_handles: WindowResizeHandles,

    mouse: Vec2,
    pub open: bool,
    pub active: bool,
    pub hover: bool,
    pub dragging: Option<Vec2>,
    pub resizing: bool,
}

impl Window {
    pub fn new(id: impl ToString, theme: WindowTheme) -> Window {
        Window {
            theme,
            id: id.to_string(),
            title: "Window".into(),
            rect: Rect::new(0.0, 0.0, 200.0, 150.0),
            info: WindowInfo::new(),
            resize_handles: WindowResizeHandles::new(),
            widget_holder: WidgetHolder::new(),
            
            open: true,
            mouse: mouse_position().into(),
            active: false,
            hover: false,
            dragging: None,
            resizing: false
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

    pub fn set_pos(&mut self, position: Vec2, action_type: impl Into<ActionType> + Clone) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.x = position.x;
            self.rect.y = position.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.x = position.x;
            self.rect.y = position.y;
        }
        
        self
    }

    pub fn set_size(&mut self, size: Vec2, action_type: impl Into<ActionType> + Clone) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.w = size.x;
            self.rect.h = size.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.w = size.x;
            self.rect.h = size.y;
        }
        
        self
    }

    pub fn close(&mut self) {
        self.open = false;
        
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

        self.widget_holder.render(&self.rect, self.info.show_titlebar, &self.theme.font);
        
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
                font: Some(&self.theme.font),
                font_size: 13,
                ..Default::default()
            },
        );

        self.draw_close_button();
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
                font: Some(&self.theme.font),
                font_size: 13,
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
        self.mouse = mouse_position().into();

        let hover = self.rect.contains(self.mouse);
        let window_action = mouse_action == MouseAction::WindowHover(self.id.clone())
            || mouse_action == MouseAction::Normal;
        self.hover = window_action && hover;

        if is_mouse_button_pressed(Left) && window_action {
            if !active_taken {
                self.active = hover;
            } else {
                self.active = false;
            }
        }

        self.update_resize_handles(window_action);
        if !self.resizing {
            self.handle_close_button(window_action);
        }
        
        if window_action {
            self.widget_holder.update(&self.rect, self.info.show_titlebar, hover, self.mouse, &self.theme.font)
        }
        
        self.handle_dragging();
        
        if let Some(start_offset) = self.dragging {
            self.set_pos(self.mouse + start_offset, ActionType::EachFrame);
            self.clamp();
        } else if self.resizing {
            self.clamp();
        }
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
            Some(&self.theme.font),
            13,
            1.0
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
            return
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

    fn update_resize_handles(&mut self, window_action: bool) {
        self.resize_handles.update(&mut self.rect, window_action);
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
    
    pub fn text(&mut self, id: impl Into<WidgetId>, label: impl ToString) -> &mut Text {
        self.widget_holder.text(id.into(), label.to_string())
    }
    
    pub fn button(&mut self, id: impl Into<WidgetId>, label: impl ToString) -> &mut Button {
        self.widget_holder.button(id.into(), label.to_string())
    }
    
    pub fn checkbox(&mut self, id: impl Into<WidgetId>, label: impl ToString, default_value: bool) -> &mut Checkbox {
        self.widget_holder.checkbox(id.into(), label.to_string(), default_value)
    }
    
    pub async fn image(&mut self, id: impl Into<WidgetId>, path: impl ToString, size: Vec2) -> &mut ImageWidget {
        self.widget_holder.image(id.into(), path.to_string(), size).await
    }
    
    pub fn slider(&mut self, id: impl Into<WidgetId>, label: impl ToString, slider_info: SliderInfo) -> &mut Slider {
        self.widget_holder.slider(id.into(), label.to_string(), slider_info)
    }
}