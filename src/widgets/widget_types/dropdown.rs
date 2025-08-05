use std::any::Any;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

pub struct Dropdown {
	pub value: String,
	items: Vec<String>,
	
	pub open: bool,
	pub hovered: bool,
	pub pressed: bool,
	pub clicked: bool,
	pub item_hovered: Option<String>,
	pub item_pressed: bool,
}

impl Dropdown {
	pub fn new(items: Vec<String>, default_value: String) -> Self {
		Self {
			items,
			value: default_value,
			
			open: false,
			hovered: false,
			pressed: false,
			clicked: false,
			item_pressed: false,
			item_hovered: None,
		}
	}
}

impl Widget for Dropdown {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&"A".repeat(self.items.iter().map(|x| x.len()).max().unwrap()),
			Some(info.font),
			13,
			1.0
		);
		
		draw_rectangle(
			0.0,
			info.rect.h,
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
			info.rect.h,
			text_dim.height + 10.0,
			text_dim.height + 10.0,
			Color::new(0.4, 0.7, 1.0, 1.0)
		);
		
		draw_line(
			text_dim.width + 15.0,
			info.rect.h + 9.0,
			text_dim.width + 20.0,
			info.rect.h + 15.0,
			2.0,
			WHITE
		);
		
		draw_line(
			text_dim.width + 20.0,
			info.rect.h + 15.0,
			text_dim.width + 24.0,
			info.rect.h + 9.0,
			2.0,
			WHITE
		);
		
		let value_dim = measure_text(
			&self.value,
			Some(info.font),
			13,
			1.0
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.value,
				(text_dim.width - value_dim.width + 10.0) / 2.0,
				info.rect.h + text_dim.height * 2.0 - 5.0,
				TextParams {
					color: WHITE,
					font: Some(info.font),
					font_size: 13,
					..Default::default()
				}
			);
		}
		
		if self.open {
			let scale = 2.0/(text_dim.width + 10.0);
			let (zoom_x, zoom_y) = (
				scale,
				scale * (text_dim.width + 10.0) / 120.0,
			);
			
			let target = render_target(text_dim.width as u32 + 10, 120);
			set_camera(&Camera2D {
				zoom: vec2(zoom_x, zoom_y),
				target: vec2(1.0 / zoom_x, 1.0 / zoom_y),
				render_target: Some(target.clone()),
				..Default::default()
			});
			
			for i in 0..self.items.len() {
				let item_text_dim = measure_text(
					&self.items[i],
					Some(info.font),
					13,
					1.0
				);
				
				if let Some(x) = &self.item_hovered {
					if x == &self.items[i] {
						draw_rectangle(
							2.0,
							(text_dim.height + 10.0) * i as f32 + 5.0,
							text_dim.width + 8.0,
							text_dim.height + 10.0,
							match self.item_pressed {
								true => Color::new(0.2, 0.4, 0.6, 1.0),
								_ => Color::new(0.15,0.15,0.15,1.0)
							}
						)
					}
				}
				
				for _ in 0..4 {
					draw_text_ex(
						&self.items[i],
						(text_dim.width - item_text_dim.width) / 2.0 + 5.0,
						(text_dim.height + 10.0) * i as f32 + text_dim.height + 10.0,
						TextParams {
							color: WHITE,
							font: Some(info.font),
							font_size: 13,
							..Default::default()
						}
					);
				}
			}
			
			set_camera(info.cam_2);
			
			draw_rectangle(
				0.0,
				info.rect.h + text_dim.height + 10.0,
				text_dim.width + 10.0,
				((text_dim.height + 10.0) * self.items.len() as f32 + 10.0).max(0.0).min(120.0),
				BLACK
			);
			
			draw_rectangle_lines(
				0.0,
				info.rect.h + text_dim.height + 10.0,
				text_dim.width + 10.0,
				((text_dim.height + 10.0) * self.items.len() as f32 + 10.0).max(0.0).min(120.0),
				2.0,
				DARKGRAY
			);
			
			draw_texture_ex(
				&target.texture,
				0.0,
				info.rect.h + text_dim.height + 10.0,
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(
						0.0,
						0.0,
						(text_dim.width + 10.0).max(0.0),
						(120f32).max(0.0),
					)),
					..Default::default()
				}
			);
			
			set_camera(info.cam_1);
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
				rect.y + rect.h,
				text_dim.width + 10.0,
				120.0
			);
			
			if drop_rect.contains(info.mouse) {
				info.mouse_action.taken = true;
				let mut base_item_rect = drop_rect.clone();
				base_item_rect.h = text_dim.height + 10.0;
				
				if is_mouse_button_pressed(Left) {
					self.item_pressed = true;
				}
				
				if is_mouse_button_released(Left) && self.item_pressed {
					self.value = self.item_hovered.clone().unwrap();
				}
				
				self.item_hovered = None;
				for i in 0..self.items.len() {
					if base_item_rect.contains(info.mouse) {
						self.item_hovered = Some(self.items[i].clone())
					}
					
					base_item_rect.y += text_dim.height + 10.0
				}
			}
		}
		
		if is_mouse_button_released(Left) {
			if self.pressed && !self.open {
				self.open = true;
			} else {
				self.open = false;
			}
			self.pressed = false;
			self.item_pressed = false;
		}
		
		Some(vec2(text_dim.width, text_dim.height + 10.0))
	}
}