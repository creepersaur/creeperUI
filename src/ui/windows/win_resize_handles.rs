use macroquad::input::MouseButton::Left;
use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use crate::ui::windows::window_theme::WindowTheme;

pub enum ResizeAxis {
    X(f32, f32),
    Y(f32, f32),
    XY(f32, f32, f32, f32),
}

pub enum ResizeHandle {
	Right, Bottom, Corner
}

pub struct WindowResizeHandles {
    pub thickness: f32,
	pub corner_size: f32,
    pub resizing: Option<ResizeAxis>,
	pub hovering_handle: Option<ResizeHandle>,
	pub opacity: f32,
}

impl WindowResizeHandles {
    pub fn new() -> Self {
        Self {
            thickness: 7.0,
			corner_size: 12.0,
			resizing: None,
			hovering_handle: None,
			opacity: 0.0,
        }
    }

    pub fn render(&self, rect: &Rect, theme: &WindowTheme) {
		let mut color = theme.resize_handle.to_vec();
		color.w *= self.opacity;
		
		let color = Color::from_vec(color);
		
		match self.hovering_handle {
			Some(ResizeHandle::Right) => {
				// RIGHT HANDLE
				draw_line(
					rect.x + rect.w,
					rect.y,
					rect.x + rect.w,
					rect.y + rect.h,
					self.thickness,
					color,
				);
				set_mouse_cursor(CursorIcon::EWResize);
			}
			
			Some(ResizeHandle::Bottom) => {
				// BOTTOM HANDLE
				draw_line(
					rect.x,
					rect.y + rect.h,
					rect.x + rect.w,
					rect.y + rect.h,
					self.thickness,
					color,
				);
				set_mouse_cursor(CursorIcon::NSResize);
			}
			
			Some(ResizeHandle::Corner) => {
				// CORNER HANDLE
				draw_rectangle(
					rect.x + rect.w - self.corner_size/2.0,
					rect.y + rect.h - self.corner_size/2.0,
					self.corner_size,
					self.corner_size,
					color,
				);
				set_mouse_cursor(CursorIcon::NWSEResize);
			}
			
			_ => {}
		}
    }

    pub fn update(&mut self, rect: &mut Rect, hover: bool, taken: bool) {
		let mouse: Vec2 = mouse_position().into();
		
		if self.resizing.is_none() {
			self.hovering_handle = None;
		}
		
		if hover && !taken {
			self.corner_handle(mouse, rect);
			self.right_handle(mouse, rect);
			self.bottom_handle(mouse, rect);
		}
		
		if is_mouse_button_released(Left) {
			self.resizing = None;
		}
		
		if self.hovering_handle.is_some() {
			self.opacity = self.opacity.lerp(1.0, 0.1);
		} else {
			self.opacity = self.opacity.lerp(0.0, 0.1);
		}
		
		match &self.resizing {
			&Some(ResizeAxis::X(start, o_width)) => {
				rect.w = o_width + (mouse.x - start);
			}
			
			&Some(ResizeAxis::Y(start, o_height)) => {
				rect.h = o_height + (mouse.y - start);
			}
			
			&Some(ResizeAxis::XY(sx, sy, ow, oh)) => {
				rect.w = ow + (mouse.x - sx);
				rect.h = oh + (mouse.y - sy);
			}
			
			_ => {}
		}
	}
	
	pub fn right_handle(&mut self, mouse: Vec2, rect: &mut Rect) {
		let handle = Rect::new(
			rect.x + rect.w - self.thickness/2.0,
			rect.y,
			self.thickness,
			rect.h
		);
		
		if handle.contains(mouse) && self.resizing.is_none() {
			self.hovering_handle = Some(ResizeHandle::Right);
			
			if is_mouse_button_pressed(Left) {
				self.resizing = Some(ResizeAxis::X(mouse.x, rect.w))
			}
		}
	}
	
	pub fn bottom_handle(&mut self, mouse: Vec2, rect: &mut Rect) {
		let handle = Rect::new(
			rect.x,
			rect.y + rect.h - self.thickness/2.0,
			rect.w,
			self.thickness
		);
		
		if handle.contains(mouse) && self.resizing.is_none() {
			self.hovering_handle = Some(ResizeHandle::Bottom);
			
			if is_mouse_button_pressed(Left) {
				self.resizing = Some(ResizeAxis::Y(mouse.y, rect.h))
			}
		}
	}
	
	pub fn corner_handle(&mut self, mouse: Vec2, rect: &mut Rect) {
		let handle = Rect::new(
			rect.x + rect.w - self.corner_size/2.0,
			rect.y + rect.h - self.corner_size/2.0,
			self.corner_size,
			self.corner_size
		);
		
		if handle.contains(mouse) && self.resizing.is_none() {
			self.hovering_handle = Some(ResizeHandle::Corner);
			
			if is_mouse_button_pressed(Left) {
				self.resizing = Some(ResizeAxis::XY(mouse.x, mouse.y, rect.w, rect.h))
			}
		}
	}
}
