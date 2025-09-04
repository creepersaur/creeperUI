use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub struct Checkbox {
    pub text: String,
    pub value: bool,
    pressed: bool,
    clicked: bool,
    hovered: bool,
}

impl Checkbox {
    pub fn new(text: String, value: bool) -> Self {
        Self {
            text,
            value,
            pressed: false,
            clicked: false,
            hovered: false,
        }
    }
}

impl Widget for Checkbox {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let text_dim = measure_text(
            &self.text.to_string(),
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );
        
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h
        };
        
        for _ in 0..4 {
            draw_text_ex(
                &self.text.to_string(),
                info.rect.x + text_dim.height + 10.0,
                info.rect.y + text_dim.height + vertical_height + 2.0,
                TextParams {
                    font: match &info.font {
                        Some(f) => Some(&f),
                        _ => None,
                    },
                    font_size: 14,
                    color: match self.hovered {
                        true => WHITE,
                        _ => Color::new(0.9, 0.9, 0.9, 0.9),
                    },
                    ..Default::default()
                },
            );
        }

        draw_rectangle(
            info.rect.x,
            info.rect.y + vertical_height,
            text_dim.height + 5.0,
            text_dim.height + 5.0,
            match (self.hovered, self.pressed) {
                (true, false) => Color::new(0.2, 0.4, 0.6, 1.0),
                (_, true) => Color::new(0.3, 0.5, 0.75, 1.0),
                _ => Color::new(0.05, 0.2, 0.4, 1.0),
            },
        );

        if self.pressed {
            draw_rectangle_lines(
                info.rect.x,
                info.rect.y + vertical_height,
                text_dim.height + 5.0,
                text_dim.height + 5.0,
                2.0,
                WHITE,
            );
        }

        if self.value {
            draw_rectangle(
                info.rect.x + 1.0,
                info.rect.y + vertical_height + 1.0,
                text_dim.height + 3.0,
                text_dim.height + 3.0,
                Color::new(0.3, 0.7, 1.0, 1.0),
            );

            draw_line(
                info.rect.x + 4.0,
                info.rect.y + vertical_height + 8.0,
                info.rect.x + 6.0,
                info.rect.y + text_dim.height + vertical_height + 3.0,
                3.0,
                WHITE,
            );

            draw_line(
                info.rect.x + 6.0,
                info.rect.y + text_dim.height + vertical_height + 3.0,
                info.rect.x + text_dim.height + 2.0,
                info.rect.y + vertical_height + 2.0,
                3.0,
                WHITE,
            )
        }

        Some(vec2(
            text_dim.width + 10.0 + text_dim.height,
            text_dim.height + 5.0,
        ))
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        let text_dim = measure_text(
            &self.text.to_string(),
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );
        
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h
        };
        
        let rect = Rect::new(
            info.rect.x + text_dim.height + 7.0,
            info.rect.y + vertical_height,
            text_dim.width + text_dim.height + 10.0,
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

        if self.clicked {
            self.value = !self.value;
        }

        Some(vec2(
            text_dim.width + text_dim.height + 10.0,
            text_dim.height + 5.0,
        ))
    }
}
