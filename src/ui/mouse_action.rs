#[derive(Clone, Debug, PartialEq)]
pub enum MouseAction {
	WindowHover(String),
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