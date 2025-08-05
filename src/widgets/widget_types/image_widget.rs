use std::any::Any;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

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
	
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		draw_texture_ex(
			&self.texture,
			0.0,
			info.rect.h,
			WHITE,
			DrawTextureParams {
				dest_size: Some(self.size),
				..Default::default()
			}
		);
		
		Some(self.size)
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		Some(self.size)
	}
}