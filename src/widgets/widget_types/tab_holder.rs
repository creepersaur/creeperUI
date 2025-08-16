use std::any::Any;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

pub struct TabHolder {
	pub value: usize,
	pub tabs: Vec<String>,
	
	pub hovered: i16,
}

impl TabHolder {
	pub fn new(tabs: Vec<String>, default_tab: usize) -> Self {
		Self {
			tabs,
			value: default_tab,
			hovered: -1,
		}
	}
}

impl Widget for TabHolder {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.value.to_string(),
			Some(info.font),
			13,
			1.0
		);
		
		// DRAW BASE
		draw_rectangle(
			-5.0,
			0.0,
			info.win_rect.w - 5.0,
			30.0,
			BLACK
		);
		
		// DRAW UNDERLINES
		let length = self.tabs.len();
		
		for i in 0..length {
			if self.value == i {
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					0.0,
					info.win_rect.w / length as f32,
					30.0,
					Color::new(0.2, 0.4, 0.7, 0.2)
				);
				draw_rectangle_lines(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					0.0,
					info.win_rect.w / length as f32,
					30.0,
					2.0,
					Color::new(0.2, 0.5, 0.7, 0.3)
				);
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 2.0,
					25.0,
					info.win_rect.w / length as f32 - 4.0,
					5.0,
					Color::new(0.2, 0.5, 0.8, 1.0)
				);
			}
			if self.hovered == i as i16 {
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					0.0,
					info.win_rect.w / length as f32,
					30.0,
					Color::new(1.0,1.0,1.0,0.1)
				);
				draw_rectangle_lines(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					0.0,
					info.win_rect.w / length as f32,
					30.0,
					2.0,
					Color::new(1.0,1.0,1.0,0.3)
				);
			}
			
			let text_dim = measure_text(&self.tabs[i], Some(info.font), 13, 1.0);
			
			for _ in 0..4 {
				draw_text_ex(
					&self.tabs[i],
					i as f32 * info.win_rect.w / length as f32 + info.win_rect.w / length  as f32 / 2.0 - text_dim.width / 2.0 - 4.0,
					18.0,
					TextParams {
						font: Some(info.font),
						font_size: 13,
						..Default::default()
					}
				);
			}
		}
		
		Some(vec2(info.win_rect.w, 30.0))
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		let length = self.tabs.len();
		
		self.hovered = -1;
		
		for i in 0..length {
			let rect = Rect::new(
				info.rect.x + i as f32 * info.win_rect.w / length as f32 - 4.0,
				info.rect.y + info.rect.h,
				 info.win_rect.w / length as f32,
				 30.0
			);
			
			if rect.contains(info.mouse) {
				self.hovered = i as i16;
				if is_mouse_button_pressed(Left) {
					self.value = i;
				}
			}
		}
		
		Some(vec2(info.win_rect.w, 30.0))
	}
}