use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub struct Button {
    pub value: String,
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(value: String) -> Self {
        Self {
            value,
            hovered: false,
            pressed: false,
            clicked: false,
        }
    }
}

impl Widget for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let text_dim = measure_text(
            &self.value.to_string(),
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            13,
            1.0,
        );

        draw_rectangle(
            0.0,
            info.rect.h,
            text_dim.width + 10.0,
            text_dim.height + 10.0,
            match (self.hovered, self.pressed) {
                (true, false) => Color::new(0.2, 0.4, 0.7, 0.9), // HOVER
                (_, true) => Color::new(0.3, 0.5, 0.8, 1.0),     // PRESSED
                _ => Color::new(0.1, 0.3, 0.5, 0.9),
            },
        );

        if self.pressed {
            draw_rectangle_lines(
                0.0,
                info.rect.h,
                text_dim.width + 10.0,
                text_dim.height + 10.0,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.7),
            );
        }

        for i in 0..4 {
            draw_text_ex(
                &self.value.to_string(),
                5.0,
                text_dim.height + info.rect.h + 5.0,
                TextParams {
                    font: match &info.font {
                        Some(f) => Some(&f),
                        _ => None,
                    },
                    font_size: 13,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        Some(vec2(text_dim.width, text_dim.height + 10.0))
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        let text_dim = measure_text(
            &self.value.to_string(),
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            13,
            1.0,
        );

        let rect = Rect::new(
            info.rect.x,
            info.rect.y + info.rect.h,
            text_dim.width + 10.0,
            text_dim.height + 10.0,
        );

        if rect.contains(info.mouse) && info.hover && !info.mouse_action.taken {
            self.hovered = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.pressed = true;
            }
        } else {
            self.hovered = false;
        }

        self.clicked = false;
        if is_mouse_button_released(MouseButton::Left) {
            if self.pressed && self.hovered {
                self.clicked = true;
            }
            self.pressed = false;
        }

        Some(vec2(text_dim.width, text_dim.height + 10.0))
    }
}
