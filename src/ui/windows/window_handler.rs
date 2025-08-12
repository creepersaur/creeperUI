use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use macroquad::miniquad::window::set_mouse_cursor;
use crate::ui::mouse_action::MouseAction;
use crate::ui::windows::window::Window;
use crate::ui::windows::window_theme::WindowTheme;
use crate::widgets::WidgetId;

pub type WindowId = u64;

pub struct WindowHandler {
	windows: HashMap<WindowId, Window>,
	safe_queue: Vec<WindowId>,
	latest_active: Vec<WindowId>,
	theme: WindowTheme,
	mouse_action: MouseAction
}

impl WindowHandler {
	pub async fn new() -> WindowHandler {
		WindowHandler {
			windows: HashMap::new(),
			safe_queue: vec![],
			latest_active: vec![],
			theme: WindowTheme::new().await,
			mouse_action: MouseAction::Normal,
		}
	}
	
	pub fn begin(&mut self, id: impl ToString) -> &mut Window {
		let win_id = create_window_id(&id.to_string());
		self.safe_queue.push(win_id);
		
		if !self.windows.contains_key(&win_id) {
			self.windows.insert(win_id, Window::new(win_id, self.theme.clone()));
			self.latest_active.insert(0, win_id);
		}
		
		let w = self.windows.get_mut(&win_id).unwrap();
		w.begin_widgets();
		w
	}
	
	pub fn update(&mut self) {
		let mut is_active = false;
		let mut active_window = None;
		self.mouse_action = MouseAction::Normal;
		
		for i in self.latest_active.clone() {
			let win = self.windows.get_mut(&i).unwrap();
			let id = win.id;
			
			if win.open {
				win.update(is_active, self.mouse_action.clone());
				
				if win.hover || win.resizing {
					self.mouse_action = MouseAction::WindowHover(id);
				}
				
				if win.active {
					is_active = true;
					active_window = Some(i);
				}
			}
		}
		
		if let Some(active) = active_window {
			let idx = self.latest_active.iter().position(|x| *x == active).unwrap();
			self.latest_active.remove(idx);
			self.latest_active.insert(0, active);
		}
	}
	
	pub fn render(&mut self) {
		let mut reversed = self.latest_active.clone();
		reversed.reverse();
		
		for i in reversed {
			let win = self.windows.get_mut(&i).unwrap();
			
			if win.open {
				win.render()
			}
		}
	}
	
	pub fn retain(&mut self) {
		self.windows.retain(|x, _| self.safe_queue.contains(x));
		self.latest_active.retain(|x| self.safe_queue.contains(x));
		self.safe_queue.clear();
		for (_, w) in self.windows.iter_mut() {
			w.end_widgets();
			if !w.info.ran_once {
				w.info.ran_once = true;
			}
		}
	}
	
	pub fn end_windows(&mut self) {
		self.update();
		self.render();
		self.retain();
	}
}

fn create_window_id(id: &str) -> WindowId {
	let mut hasher = DefaultHasher::new();
	id.hash(&mut hasher);
	
	hasher.finish()
}