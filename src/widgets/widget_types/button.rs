use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;
use std::ops::Add;

pub struct Button {
    pub value: String,
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
    
    background: Color,
}

impl Button {
    pub fn new(value: String) -> Self {
        Self {
            value,
            hovered: false,
            pressed: false,
            clicked: false,
            
            background: Color::new(0.1, 0.3, 0.5, 0.9),
        }
    }
    
    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.background = color;
        
        self
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
            14,
            1.0,
        );
        
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h
        };

        draw_rectangle(
            info.rect.x,
            info.rect.y + vertical_height,
            text_dim.width + 10.0,
            text_dim.height + 10.0,
            match (self.hovered, self.pressed) {
                (true, false) => Color::from_vec(self.background.to_vec().add(vec4(0.1, 0.1, 0.2, 0.0))), // HOVER
                (_, true) => Color::from_vec(self.background.to_vec().add(vec4(0.2, 0.2, 0.3, 0.1))),     // PRESSED
                _ => self.background,
            },
        );

        if self.pressed {
            draw_rectangle_lines(
                info.rect.x,
                info.rect.y + vertical_height,
                text_dim.width + 10.0,
                text_dim.height + 10.0,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.7),
            );
        }

        for _ in 0..4 {
            draw_text_ex(
                &self.value.to_string(),
                info.rect.x + 5.0,
                info.rect.y + text_dim.height + 5.0 + vertical_height,
                TextParams {
                    font: match &info.font {
                        Some(f) => Some(&f),
                        _ => None,
                    },
                    font_size: 14,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        Some(vec2(text_dim.width + 10.0, text_dim.height + 10.0))
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        let text_dim = measure_text(
            &self.value.to_string(),
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
            info.rect.x,
            info.rect.y + vertical_height,
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

        Some(vec2(text_dim.width + 10.0, text_dim.height + 10.0))
    }
}
