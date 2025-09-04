use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub struct Text {
    pub(crate) value: String,
}

impl Text {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Widget for Text {
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

        for _ in 0..4 {
            draw_text_ex(
                &self.value.to_string(),
                info.rect.x,
                info.rect.y + text_dim.height + 2.0 + vertical_height,
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

        Some(vec2(text_dim.width, text_dim.height))
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

        Some(vec2(text_dim.width, text_dim.height))
    }
}
