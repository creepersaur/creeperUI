use std::any::Any;
use macroquad::prelude::*;
use crate::widgets::widget::Widget;

pub struct ImageWidget {
	texture: Texture2D,
	size: Vec2,
}

impl ImageWidget {
	pub async fn new(path: String, size: Vec2) -> Self {
		Self {
			size,
			texture: load_texture(&path).await.unwrap()
		}
	}
}

impl Widget for ImageWidget {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, rect: &Rect, font: &Font) -> Option<Vec2> {
		draw_texture_ex(
			&self.texture,
			0.0,
			rect.h,
			WHITE,
			DrawTextureParams {
				dest_size: Some(self.size),
				..Default::default()
			}
		);
		
		Some(self.size)
	}
	
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) -> Option<Vec2> {
		Some(self.size)
	}
}