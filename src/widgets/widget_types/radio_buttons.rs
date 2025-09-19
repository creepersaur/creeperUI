use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub struct RadioButtons {
    pub options: Vec<String>,
    pub value: String,

    pub hovered: Option<usize>,
    pub pressed: Option<usize>,
    pub clicked: Option<usize>,

    pub padding: f32,
}

impl RadioButtons {
    pub fn new(options: Vec<String>, default_value: String) -> Self {
        Self {
            options,
            value: default_value,

            hovered: None,
            pressed: None,
            clicked: None,

            padding: 10.0,
        }
    }

    pub fn set_padding(&mut self, padding: f32) -> &mut Self {
        self.padding = padding;
        self
    }
}

impl Widget for RadioButtons {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let char_dim = measure_text(
            "A",
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );

        let mut text_y = 0.0;
        let mut text_width = 0.0;
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h,
        };

        for i in 0..self.options.len() {
            let text = &self.options[i];
            let text_dim = measure_text(
                text,
                match &info.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                14,
                1.0,
            );

            if text_dim.width > text_width {
                text_width = text_dim.width;
            }
            
            text_y += text_dim.height + self.padding;
            let hovered = match self.hovered {
                Some(t) => t == i,
                _ => false,
            };

            let pressed = self.pressed.is_some() && self.pressed.unwrap() == i;

            // DRAW CIRCLE

            draw_circle(
                info.rect.x + 10.0,
                info.rect.y + vertical_height - 8.0 + text_y - self.padding,
                8.0,
                match (hovered, pressed) {
                    (true, false) => Color::new(0.22, 0.35, 0.55, 0.9),
                    (_, true) => Color::new(0.2, 0.3, 0.5, 0.7),
                    _ => Color::new(0.2, 0.3, 0.6, 0.8),
                },
            );

            if self.value == *text {
                draw_circle(
                    info.rect.x + 10.0,
                    info.rect.y + vertical_height - 8.0 + text_y - self.padding,
                    5.0,
                    Color::new(0.3, 0.6, 0.9, 1.0),
                );
            }

            // DRAW TEXT

            for _ in 0..4 {
                draw_text_ex(
                    text,
                    info.rect.x + text_dim.height + 10.0,
                    info.rect.y + vertical_height + text_y - self.padding - 3.0,
                    TextParams {
                        font: match &info.font {
                            Some(f) => Some(&f),
                            _ => None,
                        },
                        font_size: 14,
                        color: match (hovered, pressed) {
                            (true, false) => WHITE,
                            (_, true) => WHITE.with_alpha(0.75),
                            _ => WHITE.with_alpha(0.82),
                        },
                        ..Default::default()
                    },
                );
            }
        }

        Some(vec2(text_width + 10.0 + char_dim.height, text_y - self.padding))
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        let char_dim = measure_text(
            "A",
            match &info.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );

        let mut text_y = 0.0;
        let mut text_width = 0.0;
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h,
        };

        self.hovered = None;
        self.clicked = None;

        for i in 0..self.options.len() {
            let text = &self.options[i];
            let mut text_dim = measure_text(
                text,
                match &info.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                14,
                1.0,
            );

            text_y += text_dim.height + self.padding;
            if text.len() > 0 {
                text_dim.width += 5.0;
            }
            if text_dim.width > text_width {
                text_width = text_dim.width;
            }

            let rect = Rect::new(
                info.rect.x,
                info.rect.y + vertical_height + text_y - text_dim.height - self.padding,
                text_dim.width + char_dim.height + 8.0,
                char_dim.height + 10.0,
            );

            if rect.contains(info.mouse) && info.hover && !info.mouse_action.taken {
                self.hovered = Some(i);
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.pressed = Some(i);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if self.pressed.is_some() && self.hovered.is_some() {
                self.clicked = self.pressed;
                self.value = self.options[self.clicked.unwrap()].clone();
            }
            self.pressed = None;
        }

        Some(vec2(text_width + char_dim.height + 10.0, text_y - self.padding))
    }
}
