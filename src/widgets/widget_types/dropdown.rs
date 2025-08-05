use std::any::Any;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::UpdateInfo;

pub struct Dropdown {
	pub value: String,
	items: Vec<String>,
	
	pub open: bool,
	pub hovered: bool,
	pub pressed: bool,
	pub clicked: bool,
}

impl Dropdown {
	pub fn new(items: Vec<String>, default_value: String) -> Self {
		Self {
			items,
			value: default_value,
			
			open: false,
			hovered: false,
			pressed: false,
			clicked: false
		}
	}
}

impl Widget for Dropdown {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, rect: &Rect, font: &Font, win_rect: &Rect) -> Option<Vec2> {
		let text_dim = measure_text(
			&"A".repeat(self.items.iter().map(|x| x.len()).max().unwrap()),
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
		
		draw_rectangle(
			text_dim.width + 10.0,
			rect.h,
			text_dim.height + 10.0,
			text_dim.height + 10.0,
			Color::new(0.4, 0.7, 1.0, 1.0)
		);
		
		draw_line(
			text_dim.width + 15.0,
			rect.h + 9.0,
			text_dim.width + 20.0,
			rect.h + 15.0,
			2.0,
			WHITE
		);
		
		draw_line(
			text_dim.width + 20.0,
			rect.h + 15.0,
			text_dim.width + 24.0,
			rect.h + 9.0,
			2.0,
			WHITE
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.value,
				text_dim.width / 2.0 - 5.0,
				rect.h + text_dim.height * 2.0 - 5.0,
				TextParams {
					color: WHITE,
					font: Some(font),
					font_size: 13,
					..Default::default()
				}
			);
		}
		
		Some(vec2(text_dim.width, text_dim.height + 10.0))
	}
	
	fn render_top(&self, rect: &Rect, font: &Font, win_rect: &Rect) -> Option<Vec2> {
		let text_dim = measure_text(
			&"A".repeat(self.items.iter().map(|x| x.len()).max().unwrap()),
			Some(font),
			13,
			1.0
		);
		
		if self.open {
			draw_rectangle(
				0.0,
				rect.h + text_dim.height + 10.0,
				text_dim.width + 10.0,
				120.0,
				GRAY
			);
		}
		
		Some(vec2(text_dim.width, text_dim.height + 10.0))
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&"A".repeat(self.items.iter().map(|x| x.len()).max().unwrap()),
			Some(info.font),
			13,
			1.0
		);
		
		let rect = Rect::new(
			info.rect.x,
			info.rect.y + info.rect.h,
			text_dim.width + text_dim.height + 20.0,
			text_dim.height + 10.0
		);
		
		if rect.contains(info.mouse) && !info.mouse_action.taken {
			self.hovered = true;
			if is_mouse_button_pressed(Left) {
				self.pressed = true;
			}
		} else {
			self.hovered = false;
		}
		
		if self.open {
			let target = render_target(rect.w as u32, info.win_rect.h as u32);
			
			let drop_rect = Rect::new(
				rect.x,
				rect.y + rect.h + text_dim.height,
				text_dim.width + 10.0,
				120.0
			);
			
			if drop_rect.contains(info.mouse) {
				info.mouse_action.taken = true;
			}
			
			draw_texture_ex(
				&target.texture,
				rect.x,
				rect.y,
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(
						0.0,
						0.0,
						rect.w.max(0.0),
						rect.h.max(0.0),
					)),
					..Default::default()
				},
			);
		}
		
		if is_mouse_button_released(Left) {
			if self.pressed && !self.open {
				self.open = true;
			} else {
				self.open = false;
			}
			self.pressed = false;
		}
		
		Some(vec2(text_dim.width, text_dim.height + 10.0))
	}
}