use macroquad::prelude::*;

#[derive(Default)]
pub struct WindowInfo {
	pub close_button_rect: Rect,
	pub close_button_hovered: bool,
	pub close_button_pressed: bool,
	pub close_color: Color,
	pub ran_once: bool,
	pub min_size: Vec2,
}

impl WindowInfo {
	pub fn new() -> WindowInfo {
		WindowInfo {
			..Default::default()
		}
	}
}