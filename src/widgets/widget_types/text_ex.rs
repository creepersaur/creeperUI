use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::math::u16;
use macroquad::prelude::*;
use std::any::Any;

pub struct TextEx {
    pub(crate) value: String,
    pub color: Color,
    pub font_size: u16,
    pub font: Option<Font>,
}

impl TextEx {
    pub fn new(value: String, color: Color, font_size: u16, font: Option<Font>) -> Self {
        Self {
            value,
            color,
            font_size,
            font,
        }
    }
}

impl Widget for TextEx {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let font = match &self.font {
            None => match &info.font {
                Some(f) => Some(f),
                _ => None,
            },
            custom_font => Some(&custom_font.clone().unwrap()),
        };

        let text_dim = measure_text(&self.value.to_string(), font, self.font_size, 1.0);

        for _ in 0..4 {
            draw_text_ex(
                &self.value.to_string(),
                0.0,
                text_dim.height + info.rect.h,
                TextParams {
                    font,
                    font_size: self.font_size,
                    color: self.color,
                    ..Default::default()
                },
            );
        }

        Some(vec2(text_dim.width, text_dim.height))
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        let font = match &self.font {
            None => match &info.font {
                Some(f) => Some(f),
                _ => None,
            },
            custom_font => Some(&custom_font.clone().unwrap()),
        };

        let text_dim = measure_text(&self.value.to_string(), font, self.font_size, 1.0);

        Some(vec2(text_dim.width, text_dim.height))
    }
}
