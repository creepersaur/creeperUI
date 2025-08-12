use crate::ui::windows::window_handler::WindowId;

#[derive(Clone, Debug, PartialEq)]
pub enum MouseAction {
	WindowHover(WindowId),
	Normal
}

#[derive(Clone, Debug, PartialEq)]
pub struct WidgetAction {
	pub taken: bool,
}

impl WidgetAction {
	pub fn new() -> Self {
		Self {
			taken: false
		}
	}
}