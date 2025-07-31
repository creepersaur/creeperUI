use std::any::Any;
use macroquad::prelude::*;
use crate::widgets::widget::Widget;

pub struct Button {
	value: Box<&'static dyn ToString>,
	pub hovered: bool,
	pub pressed: bool,
	pub clicked: bool,
}

impl Button {
	pub fn new(value: Box<&'static dyn ToString>) -> Self {
		Self {
			value,
			hovered: false,
			pressed: false,
			clicked: false,
		}
	}
}

impl Widget for Button {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, rect: &Rect, font: &Font) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(font),
			13,
			1.0
		);
		
		draw_rectangle(
			0.0,
			rect.h,
			text_dim.width + 10.0,
			text_dim.height + 10.0,
			match (self.hovered, self.pressed) {
				(true, false) => Color::new(0.2, 0.4, 0.7, 0.9),    // HOVER
				(_, true) => Color::new(0.3, 0.5, 0.8, 1.0),        // PRESSED
				_ => Color::new(0.1, 0.3, 0.5, 0.9)
			}
		);
		
		if self.pressed {
			draw_rectangle_lines(
				0.0,
				rect.h,
				text_dim.width + 10.0,
				text_dim.height + 10.0,
				2.0,
				Color::new(1.0,1.0,1.0,0.7)
			);
		}
		
		for i in 0..4 {
			draw_text_ex(
				&self.value.to_string(),
				5.0,
				text_dim.height + rect.h + 5.0,
				TextParams {
					font: Some(font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		Some(vec2(text_dim.width, text_dim.height + 15.0))
	}
	
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(font),
			13,
			1.0
		);
		
		let rect = Rect::new(
			rect.x,
			rect.y + rect.h,
			text_dim.width + 10.0,
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
		
		Some(vec2(text_dim.width, text_dim.height + 15.0))
	}
}