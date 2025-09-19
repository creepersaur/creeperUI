use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;
use std::ops::{Add, Deref};
use crate::misc::rounded_rect::{draw_rounded_rect, draw_rounded_rect_stroke};

pub struct Button {
    pub value: String,
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
    
    background: Color,
    foreground: Color,
}

impl Button {
    pub fn new(value: String) -> Self {
        Self {
            value,
            hovered: false,
            pressed: false,
            clicked: false,
            
            background: Color::new(0.09, 0.25, 0.45, 1.0),
            foreground: WHITE,
        }
    }
    
    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.background = color;
        self
    }
    
    pub fn set_foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;
        self
    }
}

impl Deref for Button {
    type Target = bool;
    
    fn deref(&self) -> &Self::Target {
        &self.clicked
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

        if self.pressed {
            draw_rounded_rect_stroke(
                info.rect.x,
                info.rect.y + vertical_height + 5.0,
                text_dim.width + 10.0,
                text_dim.height + 10.0,
                3.0,
                1.0,
                Color::new(1.0, 1.0, 1.0, 1.0),
                match (self.hovered, self.pressed) {
                    (true, false) => Color::from_vec(self.background.to_vec().add(vec4(0.13, 0.13, 0.13, 0.0))), // HOVER
                    (_, true) => Color::from_vec(self.background.to_vec().add(vec4(0.25, 0.25, 0.25, 0.1))),     // PRESSED
                    _ => self.background,
                },
            );
        } else {
            draw_rounded_rect(
                info.rect.x,
                info.rect.y + vertical_height + 5.0,
                text_dim.width + 10.0,
                text_dim.height + 10.0,
                3.0,
                match (self.hovered, self.pressed) {
                    (true, false) => Color::from_vec(self.background.to_vec().add(vec4(0.13, 0.13, 0.13, 0.0))), // HOVER
                    (_, true) => Color::from_vec(self.background.to_vec().add(vec4(0.25, 0.25, 0.25, 0.1))),     // PRESSED
                    _ => self.background,
                },
            );
        }

        for _ in 0..4 {
            draw_text_ex(
                &self.value.to_string(),
                info.rect.x + 5.0,
                info.rect.y + text_dim.height * 2.0 + vertical_height - 3.0,
                TextParams {
                    font: match &info.font {
                        Some(f) => Some(&f),
                        _ => None,
                    },
                    font_size: 14,
                    color: self.foreground,
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
            info.rect.y + vertical_height + 5.0,
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
