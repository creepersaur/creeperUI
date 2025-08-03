use std::any::Any;
use macroquad::prelude::*;
use crate::widgets::widget::Widget;

pub struct Checkbox {
	pub text: String,
	pub value: bool,
	pressed: bool,
	clicked: bool,
	hovered: bool,
}

impl Checkbox {
	pub fn new(text: String, value: bool) -> Self {
		Self {
			text, value,
			pressed: false,
			clicked: false,
			hovered: false
		}
	}
}

impl Widget for Checkbox {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, rect: &Rect, font: &Font) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.text.to_string(),
			Some(font),
			13,
			1.0
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.text.to_string(),
				text_dim.height + 10.0,
				text_dim.height + rect.h + 2.0,
				TextParams {
					font: Some(font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		draw_rectangle(
			0.0,
			rect.h,
			text_dim.height + 5.0,
			text_dim.height + 5.0,
			match (self.hovered, self.pressed) {
				(true, false) => Color::new(0.2, 0.4, 0.6, 1.0),
				(_, true) => Color::new(0.3, 0.5, 0.75, 1.0),
				_ => Color::new(0.05, 0.2, 0.4, 1.0)
			}
		);
		
		if self.pressed {
			draw_rectangle_lines(
				0.0,
				rect.h,
				text_dim.height + 5.0,
				text_dim.height + 5.0,
				2.0,
				WHITE
			);
		}
		
		if self.value {
			draw_rectangle(
				1.0,
				rect.h + 1.0,
				text_dim.height + 3.0,
				text_dim.height + 3.0,
				Color::new(0.3, 0.7, 1.0, 1.0)
			);
			
			draw_line(
				4.0,
				rect.h + 8.0,
				6.0,
				text_dim.height + rect.h + 3.0,
				3.0,
				WHITE
			);
			
			draw_line(
				6.0,
				text_dim.height + rect.h + 3.0,
				text_dim.height + 2.0,
				rect.h + 2.0,
				3.0,
				WHITE
			)
		}
		
		Some(vec2(text_dim.width + 10.0 + text_dim.height, text_dim.height + 10.0))
	}
	
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.text.to_string(),
			Some(font),
			13,
			1.0
		);
		
		let rect = Rect::new(
			rect.x,
			rect.y + rect.h,
			text_dim.width + text_dim.height + 10.0,
			text_dim.height + 10.0
		);
		
		if rect.contains(mouse) && hover {
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
		
		if self.clicked {
			self.value = !self.value;
		}
		
		Some(vec2(text_dim.width + text_dim.height + 10.0, text_dim.height + 10.0))
	}
}