use std::any::Any;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

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
	
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(info.font),
			13,
			1.0
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.value.to_string(),
				0.0,
				text_dim.height + info.rect.h,
				TextParams {
					font: Some(info.font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		Some(vec2(text_dim.width, text_dim.height))
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(info.font),
			13,
			1.0
		);
		
		Some(vec2(text_dim.width, text_dim.height))
	}
}