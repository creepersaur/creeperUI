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
        draw_texture_ex(
            &self.texture,
            0.0,
            info.rect.h,
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
