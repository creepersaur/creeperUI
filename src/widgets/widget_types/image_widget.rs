use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub struct ImageWidget {
    pub texture: Texture2D,
    pub size: Option<Vec2>,
}

impl ImageWidget {
    pub async fn new(path: String, size: Option<Vec2>) -> Self {
        Self {
            size,
            texture: load_texture(&path).await.unwrap(),
        }
    }
}

impl Widget for ImageWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        let vertical_height = match info.same_line {
            true => 0.0,
            _ => info.rect.h
        };
        
        draw_texture_ex(
            &self.texture,
            info.rect.x,
            info.rect.y + vertical_height,
            WHITE,
            DrawTextureParams {
                dest_size: self.size,
                ..Default::default()
            },
        );

        Some(self.size.unwrap_or(self.texture.size()))
    }

    fn update(&mut self, _: &mut UpdateInfo) -> Option<Vec2> {
        Some(self.size.unwrap_or(self.texture.size()))
    }
}
