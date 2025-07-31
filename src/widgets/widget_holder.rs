use std::collections::{HashMap, HashSet};
use macroquad::prelude::*;
use crate::widgets::*;
use indexmap::IndexSet;
use owo_colors::OwoColorize;

pub struct WidgetHolder {
	target: RenderTarget,
	widgets: HashMap<String, Box<dyn Widget>>,
	frame_ids: IndexSet<String>
}

impl WidgetHolder {
	pub fn new() -> Self {
		Self {
			target: render_target(200, 200),
			widgets: HashMap::new(),
			frame_ids: IndexSet::new(),
		}
	}
	
	pub fn reset(&mut self) {
		self.frame_ids.clear();
	}
	
	pub fn retain(&mut self) {
		self.widgets.retain(|k, _| self.frame_ids.contains(k));
	}
	
	pub fn render(&self, rect: &Rect, font: &Font) {
		let zoom = 0.01;
		let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 35.0, 0.0, 0.0);
		
		set_camera(&Camera2D {
			zoom: vec2(zoom, zoom),
			target: vec2(1.0/zoom, 1.0/zoom),
			render_target: Some(self.target.clone()),
			..Default::default()
		});
		
		clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
		
		for i in self.frame_ids.iter() {
			let widget_size  = self.widgets.get(i).unwrap().render(&holder_rect, font);
			
			if let Some(size) = widget_size {
				holder_rect.h += size.y;
				if holder_rect.w < size.x {
					holder_rect.w = size.x
				}
			}
		}
		
		set_default_camera();
		draw_texture_ex(&self.target.texture, rect.x + 5.0, rect.y + 35.0, WHITE, DrawTextureParams{
			source: Some(Rect::new(0.0, 0.0, (rect.w - 5.0).max(0.0), (rect.h - 35.0).max(0.0))),
			..Default::default()
		});
	}
	
	pub fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) {
		let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 35.0, 0.0, 0.0);
		
		for i in self.frame_ids.iter() {
			let widget_size  = self.widgets.get_mut(i).unwrap().update(&holder_rect, hover, mouse, font);
			
			if let Some(size) = widget_size {
				holder_rect.h += size.y;
				if holder_rect.w < size.x {
					holder_rect.w = size.x
				}
			}
		}
	}
}

////////////////////////////////////////////////////////
// WIDGET CREATION
////////////////////////////////////////////////////////

impl WidgetHolder {
	pub fn text(&mut self, id: WidgetId, label: &'static impl ToString) -> &mut Text {
		let mut new_id = String::from("Text:");
		
		new_id.push_str(match id {
			WidgetId::Auto => label.to_string(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("Widget with id/label: {new_id} already exists. Please give a unique explicit ID.");
		}
		
		let w = Text::new(Box::new(label));
		self.widgets.insert(new_id.clone(), Box::new(w));
		self.frame_ids.insert(new_id.clone());
		
		self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap()
	}
	
	pub fn button(&mut self, id: WidgetId, label: &'static impl ToString) -> &mut Button {
		let mut new_id = String::from("Button:");
		
		new_id.push_str(match id {
			WidgetId::Auto => label.to_string(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("{} Widget with id/label: {new_id} already exists. Please give a unique explicit ID.", "Error:".red());
		}
		
		if !self.widgets.contains_key(&new_id) {
			let w = Button::new(Box::new(label));
			self.widgets.insert(new_id.clone(), Box::new(w));
		}
		self.frame_ids.insert(new_id.clone());
		
		self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap()
	}
}