use std::any::Any;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

pub struct TabHolder {
	pub value: usize,
	pub tabs: Vec<String>,
	
	holdable: bool,
	pub hovered: i16,
	pub tab_pressed: i16,
	pub pressed: bool,
}

impl TabHolder {
	pub fn new(tabs: Vec<String>, default_tab: usize) -> Self {
		Self {
			tabs,
			value: default_tab,
			
			holdable: false,
			hovered: -1,
			tab_pressed: -1,
			pressed: false,
		}
	}
	
	pub fn allow_holding(&mut self) -> &mut Self {
		self.holdable = true;
		self
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
			info.rect.h,
			info.win_rect.w - 5.0,
			30.0,
			BLACK
		);
		
		// DRAW UNDERLINES
		let length = self.tabs.len();
		
		for i in 0..length {
			if self.tab_pressed == i as i16 {
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					info.rect.h,
					info.win_rect.w / length as f32,
					30.0,
					Color::new(0.2, 0.55, 0.85, 0.2)
				);
			}
			if self.value == i {
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					info.rect.h,
					info.win_rect.w / length as f32,
					30.0,
					Color::new(0.2, 0.4, 0.7, 0.2)
				);
				draw_rectangle_lines(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					info.rect.h,
					info.win_rect.w / length as f32,
					30.0,
					2.0,
					Color::new(0.2, 0.5, 0.7, 0.3)
				);
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 2.0,
					info.rect.h + 25.0,
					info.win_rect.w / length as f32 - 4.0,
					5.0,
					Color::new(0.2, 0.5, 0.8, 1.0)
				);
			}
			if self.hovered == i as i16 {
				draw_rectangle(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					info.rect.h,
					info.win_rect.w / length as f32,
					30.0,
					Color::new(1.0,1.0,1.0,0.1)
				);
				draw_rectangle_lines(
					i as f32 * info.win_rect.w / length as f32 - 4.0,
					info.rect.h,
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
					info.rect.h + 18.0,
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
		self.tab_pressed = -1;
		
		let holder_rect = Rect::new(
			info.rect.x,
			info.rect.y + info.rect.h,
			info.win_rect.w,
			30.0
		);
		
		for i in 0..length {
			let rect = Rect::new(
				info.rect.x + i as f32 * info.win_rect.w / length as f32 - 4.0,
				info.rect.y + info.rect.h,
				 info.win_rect.w / length as f32,
				 30.0
			);
			
			if rect.contains(info.mouse) {
				self.hovered = i as i16;
				if (self.holdable && self.pressed) || is_mouse_button_pressed(Left) {
					self.value = i;
					self.tab_pressed = i as i16;
				}
			}
		}
		
		if holder_rect.contains(info.mouse) && is_mouse_button_pressed(Left) {
			self.pressed = true;
		} else if is_mouse_button_released(Left) {
			self.pressed = false;
		}
		
		Some(vec2(info.win_rect.w, 30.0))
	}
}