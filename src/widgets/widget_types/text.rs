use std::any::Any;
use macroquad::prelude::*;
use crate::widgets::widget::Widget;

pub struct Text {
	pub(crate) value: String
}

impl Text {
	pub fn new(value: String) -> Self {
		Self { value }
	}
}

impl Widget for Text {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, rect: &Rect, font: &Font, win_rect: &Rect) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(font),
			13,
			1.0
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.value.to_string(),
				0.0,
				text_dim.height + rect.h,
				TextParams {
					font: Some(font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		Some(vec2(text_dim.width, text_dim.height))
	}
	
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font, win_rect: &Rect) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(font),
			13,
			1.0
		);
		
		Some(vec2(text_dim.width, text_dim.height))
	}
}