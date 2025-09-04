use std::any::Any;
use arboard::Clipboard;
use macroquad::input::MouseButton::Left;
use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use crate::Widget;
use crate::widget_holder::{RenderInfo, UpdateInfo};

pub struct TextBox {
	pub value: String,
	pub caret: usize,
	pub label: Option<String>,
	
	selection_start: i32,
	selection_end: i32,
	
	pub editing: bool,
	hovered: bool,
	pressed: bool,
	
	// repeat state
	last_keycode: Option<KeyCode>,
	last_char: Option<char>,
	key_repeat_timer: f32,
	repeat_delay: f32,
	repeat_interval: f32,
	
	// acceleration
	last_repeat_count: u32,
	min_repeat_interval: f32,
	acceleration: f32,
}

impl TextBox {
	pub fn new(default_text: String, label: Option<String>) -> Self {
		Self {
			label,
			value: default_text,
			
			caret: 0,
			hovered: false,
			pressed: false,
			editing: false,
			
			selection_start: -1,
			selection_end: -1,
			
			last_keycode: None,
			last_char: None,
			key_repeat_timer: 0.0,
			repeat_delay: 0.18,
			repeat_interval: 0.05,
			
			last_repeat_count: 0,
			min_repeat_interval: 0.01,
			acceleration: 0.85,
		}
	}
}

impl Widget for TextBox {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
		self
	}

	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		let font = info.font.into();
		let char_dim = measure_text("A", font, 14, 1.0);
		let caret_dim = measure_text(&self.value[0..self.caret], font, 14, 1.0);
		let text_dim = measure_text(&self.value, font, 14, 1.0);
		
		let label_width = match &self.label {
			Some(s) => measure_text(s, font, 14, 1.0).width + 10.0,
			_ => 0.0
		};
		
		draw_rectangle(
			info.rect.x + label_width,
			info.rect.y + info.rect.h,
			(text_dim.width + 10.0).max(100.0),
			char_dim.height + 10.0,
			match self.hovered {
				true => Color::new(1.0, 1.0, 1.0, 0.4),
				_ => Color::new(1.0, 1.0, 1.0, 0.2),
			},
		);
		
		draw_rectangle_lines(
			info.rect.x + label_width,
			info.rect.y + info.rect.h,
			(text_dim.width + 10.0).max(100.0),
			char_dim.height + 10.0,
			2.0,
			match self.hovered {
				true => Color::new(0.1, 0.47, 0.95, 0.8),
				_ => WHITE.with_alpha(0.5)
			}
		);
		
		if self.editing {
			draw_rectangle_lines(
				info.rect.x + label_width,
				info.rect.y + info.rect.h,
				(text_dim.width + 10.0).max(100.0),
				char_dim.height + 10.0,
				2.0,
				Color::new(0.2, 0.6, 1.0, 1.0),
			);
			
			// DRAW CARET
			draw_line(
				info.rect.x + label_width + caret_dim.width + 5.0,
				info.rect.y + info.rect.h + 2.0,
				info.rect.x + label_width + caret_dim.width + 5.0,
				info.rect.y + info.rect.h +char_dim.height + 8.0,
				1.0,
				WHITE,
			);
			
			// DRAW SELECTION
			if self.selection_start > -1 && self.selection_end > -1 {
				let start = self.selection_start.min(self.selection_end);
				let end = self.selection_start.max(self.selection_end);
				
				let start_pos =
					measure_text(&self.value[0..start as usize], font, 14, 1.0).width;
				let length = measure_text(&self.value[0..end as usize], font, 14, 1.0)
					.width
					- start_pos;
				
				draw_rectangle(
					info.rect.x + label_width + 5.0 + start_pos,
					info.rect.y + info.rect.h + 2.0,
					length,
					char_dim.height + 6.0,
					DARKBLUE.with_alpha(0.88),
				);
			}
		}
		
		for _ in 0..4 {
			draw_text_ex(
				&self.value,
				info.rect.x + label_width + 5.0,
				info.rect.y + info.rect.h + char_dim.height + 4.0,
				TextParams {
					font,
					font_size: 14,
					color: WHITE,
					..Default::default()
				},
			);
		}
		
		// DRAW OPTIONAL LABEL
		if let Some(s) = &self.label {
			for _ in 0..4 {
				draw_text_ex(
					s,
					info.rect.x,
					info.rect.y + info.rect.h + char_dim.height + 4.0,
					TextParams {
						font,
						font_size: 14,
						color: WHITE,
						..Default::default()
					},
				);
			}
		}
		
		Some(vec2((text_dim.width + 10.0).max(100.0) + label_width, char_dim.height + 10.0))
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		let mut clipboard = Clipboard::new().unwrap();
		let dt = get_frame_time();
		
		let font = info.font.into();
		let char_dim = measure_text("A", font, 14, 1.0);
		let text_dim = measure_text(&self.value, font, 14, 1.0);
		let label_width = match &self.label {
			Some(s) => measure_text(s, font, 14, 1.0).width + 10.0,
			_ => 0.0
		};
		
		let rect = Rect::new(
			label_width + info.rect.x,
			info.rect.y + info.rect.h,
			(text_dim.width + 10.0).max(100.0),
			char_dim.height + 10.0,
		);
		
		let size = Some(vec2(rect.w + label_width, rect.h));
		
		if !info.mouse_action.taken && rect.contains(mouse_position().into()) {
			self.hovered = true;
			set_mouse_cursor(CursorIcon::Text);
		} else {
			self.hovered = false;
		}
		
		if info.mouse_action.taken {
			self.editing = false;
		}
		
		self.pressed = false;
		if !info.mouse_action.taken && is_mouse_button_pressed(Left) {
			if self.hovered {
				self.pressed = true;
				self.editing = true;
				
				self.selection_start = 0;
				self.selection_end = self.value.len() as i32;
				self.caret = self.value.len();
			} else {
				self.editing = false;
			}
		}
		
		if self.editing {
			info.mouse_action.taken = true;
		}
		
		// If we stop editing or mouse click outside, reset repeat state
		if !self.editing {
			self.last_keycode = None;
			self.last_char = None;
			self.key_repeat_timer = 0.0;
			self.last_repeat_count = 0;
			return size;
		}
		
		if is_key_down(KeyCode::LeftControl) {
			if is_key_pressed(KeyCode::A) {
				self.selection_start = 0;
				self.selection_end = self.value.len() as i32;
				self.caret = self.value.len();
			}
			
			if is_key_pressed(KeyCode::V) {
				if let Ok(text) = clipboard.get_text() {
					println!("paste: {text}");
					
					self.value.insert_str(self.caret, &text);
					self.caret += text.len();
				}
			}
			
			if is_key_pressed(KeyCode::C) {
				if self.selection_start >= 0 && self.selection_end >= 0 {
					let start = self.selection_start.min(self.selection_end) as usize;
					let end = self.selection_start.max(self.selection_end) as usize;
					let text = &self.value[start..end];
					
					clipboard.set_text(text).unwrap();
				}
			}
			
			return size;
		}
		
		// --- immediate press handling ---
		if is_key_pressed(KeyCode::Backspace) {
			if !self.value.is_empty() {
				if self.selection_start > -1 {
					let start = self.selection_start.min(self.selection_end);
					let end = self.selection_start.max(self.selection_end);
					
					for _ in start..end {
						if self.value.len() > 0 {
							self.value.remove(start as usize);
						}
					}
					
					self.selection_start = -1;
					self.selection_end = -1;
				} else {
					if self.value.len() > 0 {
						self.value.remove(self.caret - 1);
						self.caret = (self.caret as i32 - 1).max(0) as usize;
					}
				}
			}
			
			if self.selection_start == self.selection_end {
				self.selection_start = -1;
			}
			
			self.last_keycode = Some(KeyCode::Backspace);
			self.last_char = None;
			self.last_repeat_count = 0;
			self.key_repeat_timer = self.repeat_delay;
			
			self.caret = self.caret.clamp(0, self.value.len());
			
			return size;
		}
		
		// --- immediate press handling ---
		if is_key_pressed(KeyCode::Left) {
			if is_key_down(KeyCode::LeftShift) {
				if self.selection_start < 0 {
					self.selection_start = self.caret as i32;
					self.selection_end = self.caret as i32 - 1;
				} else {
					self.selection_end = self.caret as i32 - 1;
				}
				self.selection_start = self.selection_start.clamp(0, self.value.len() as i32);
				self.selection_end = self.selection_end.clamp(0, self.value.len() as i32);
			} else {
				self.selection_start = -1;
			}
			
			if self.selection_start == self.selection_end {
				self.selection_start = -1;
			}
			
			if self.caret > 0 {
				self.caret -= 1;
			}
			self.last_keycode = Some(KeyCode::Left);
			self.last_char = None;
			self.last_repeat_count = 0;
			self.key_repeat_timer = self.repeat_delay;
			
			self.caret = self.caret.clamp(0, self.value.len());
			
			return size;
		}
		
		// --- immediate press handling ---
		if is_key_pressed(KeyCode::Right) {
			if is_key_down(KeyCode::LeftShift) {
				if self.selection_start < 0 {
					self.selection_start = self.caret as i32;
					self.selection_end = self.caret as i32 + 1;
				} else {
					self.selection_end = self.caret as i32 + 1;
				}
				self.selection_start = self.selection_start.clamp(0, self.value.len() as i32);
				self.selection_end = self.selection_end.clamp(0, self.value.len() as i32);
			} else {
				self.selection_start = -1;
			}
			
			if self.selection_start == self.selection_end {
				self.selection_start = -1;
			}
			
			if self.caret < self.value.len() {
				self.caret += 1;
			}
			self.last_keycode = Some(KeyCode::Right);
			self.last_char = None;
			self.last_repeat_count = 0;
			self.key_repeat_timer = self.repeat_delay;
			
			self.caret = self.caret.clamp(0, self.value.len());
			
			return size;
		}
		
		if let Some(c) = get_char_pressed() {
			if (c.is_ascii_alphanumeric() || c.is_ascii_whitespace() || c.is_ascii_punctuation())
				&& c != '\t'
				&& c != '\n'
			{
				if !self.value.is_empty() && self.selection_start > -1 {
					let start = self.selection_start.min(self.selection_end);
					let end = self.selection_start.max(self.selection_end);
					
					for _ in start..end {
						if self.value.len() > 0 {
							self.value.remove(start as usize);
						}
					}
					
					self.selection_start = -1;
					self.selection_end = -1;
				}
				
				self.value.push(c);
				self.caret = self.value.len();
				
				self.last_char = Some(c);
				self.last_keycode = get_last_key_pressed();
				self.last_repeat_count = 0;
				self.key_repeat_timer = self.repeat_delay;
			}
			return size;
		}
		
		// --- repeats with acceleration ---
		if let Some(k) = self.last_keycode {
			if is_key_down(k) {
				self.key_repeat_timer -= dt;
				if self.key_repeat_timer <= 0.0 {
					// perform repeat action
					match k {
						KeyCode::Backspace => {
							if !self.value.is_empty() {
								self.value.remove(self.caret - 1);
								self.caret = self.value.len();
							}
						}
						
						KeyCode::Left => {
							if is_key_down(KeyCode::LeftShift) {
								if self.selection_start < 0 {
									self.selection_start = self.caret as i32;
									self.selection_end = self.caret as i32 - 1;
								} else {
									self.selection_end = self.caret as i32 - 1;
								}
								self.selection_start =
									self.selection_start.clamp(0, self.value.len() as i32);
								self.selection_end =
									self.selection_end.clamp(0, self.value.len() as i32);
							} else {
								self.selection_start = -1;
							}
							
							if self.selection_start == self.selection_end {
								self.selection_start = -1;
							}
							
							if self.caret > 0 {
								self.caret -= 1;
							}
						}
						
						KeyCode::Right => {
							if is_key_down(KeyCode::LeftShift) {
								if self.selection_start < 0 {
									self.selection_start = self.caret as i32;
									self.selection_end = self.caret as i32 + 1;
								} else {
									self.selection_end = self.caret as i32 + 1;
								}
								self.selection_start =
									self.selection_start.clamp(0, self.value.len() as i32);
								self.selection_end =
									self.selection_end.clamp(0, self.value.len() as i32);
							} else {
								self.selection_start = -1;
							}
							
							if self.selection_start == self.selection_end {
								self.selection_start = -1;
							}
							
							if self.caret < self.value.len() {
								self.caret += 1;
							}
						}
						
						_ => {
							if let Some(ch) = self.last_char {
								if (ch.is_ascii_alphanumeric()
									|| ch.is_ascii_whitespace()
									|| ch.is_ascii_punctuation())
									&& ch != '\t'
									&& ch != '\n'
								{
									self.value.push(ch);
									self.caret = self.value.len();
								}
							}
						}
					}
					
					self.last_repeat_count = self.last_repeat_count.saturating_add(1);
					let mut next = self.repeat_interval
						* self.acceleration.powf(self.last_repeat_count as f32);
					if next < self.min_repeat_interval {
						next = self.min_repeat_interval;
					}
					self.key_repeat_timer += next;
				}
			} else {
				// key released -> clear repeat state
				self.last_keycode = None;
				self.last_char = None;
				self.key_repeat_timer = 0.0;
				self.last_repeat_count = 0;
			}
		}
		
		self.caret = self.caret.clamp(0, self.value.len());
		
		if self.selection_start > -1 {
			self.selection_start = self.selection_start.clamp(0, self.value.len() as i32);
		}
		if self.selection_end > -1 {
			self.selection_end = self.selection_end.clamp(0, self.value.len() as i32);
		}
		
		size
	}
}