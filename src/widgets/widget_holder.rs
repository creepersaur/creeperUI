use std::collections::{HashMap, HashSet};
use macroquad::prelude::*;
use crate::widgets::*;
use indexmap::IndexSet;
use owo_colors::OwoColorize;

pub struct WidgetHolder {
	widgets: HashMap<String, Box<dyn Widget>>,
	frame_ids: IndexSet<String>
}

impl WidgetHolder {
	pub fn new() -> Self {
		Self {
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
	
	pub fn render(&self, rect: &Rect, show_titlebar: bool, font: &Font) {
		let scale = 0.01;
		
		let title_thickness = match show_titlebar {
			false => 0.0,
			_ => 30.0
		};
		let (zoom_x, zoom_y) = (scale / rect.w * 200.0, scale / (rect.h - title_thickness) * 200.0);
		let target = render_target(rect.w as u32, (rect.h - title_thickness) as u32);
		let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 5.0 + title_thickness, 0.0, 0.0);
		
		set_camera(&Camera2D {
			zoom: vec2(zoom_x, zoom_y),
			target: vec2(1.0/zoom_x, 1.0/zoom_y),
			render_target: Some(target.clone()),
			..Default::default()
		});
		
		clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
		
		for i in self.frame_ids.iter() {
			let widget_size  = self.widgets.get(i).unwrap().render(&holder_rect, font);
			
			if let Some(size) = widget_size {
				holder_rect.h += size.y + 5.0;
				if holder_rect.w < size.x {
					holder_rect.w = size.x
				}
			}
		}
		
		set_default_camera();
		draw_texture_ex(&target.texture, rect.x + 5.0, rect.y + 5.0 + title_thickness, WHITE, DrawTextureParams{
			source: Some(Rect::new(0.0, 0.0, (rect.w - 5.0).max(0.0), (rect.h - 5.0 - title_thickness).max(0.0))),
			..Default::default()
		});
	}
	
	pub fn update(&mut self, rect: &Rect, show_titlebar: bool, hover: bool, mouse: Vec2, font: &Font) {
		let title_thickness = match show_titlebar {
			false => 0.0,
			_ => 30.0
		};
		let mut holder_rect = Rect::new(rect.x + 5.0, rect.y + 5.0 + title_thickness, 0.0, 0.0);
		
		for i in self.frame_ids.iter() {
			let widget_size  = self.widgets.get_mut(i).unwrap().update(&holder_rect, hover, mouse, font);
			
			if let Some(size) = widget_size {
				holder_rect.h += size.y + 5.0;
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
	pub fn text(&mut self, id: WidgetId, label: String) -> &mut Text {
		let mut new_id = String::from("Text:");
		
		new_id.push_str(match id {
			WidgetId::Auto => label.clone(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("Widget with id/label: {new_id} already exists. Please give a unique explicit ID.");
		}
		
		let w = Text::new(label);
		self.widgets.insert(new_id.clone(), Box::new(w));
		self.frame_ids.insert(new_id.clone());
		
		self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap()
	}
	
	pub fn button(&mut self, id: WidgetId, label: String) -> &mut Button {
		let mut new_id = String::from("Button:");
		
		new_id.push_str(match id {
			WidgetId::Auto => label.clone(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("{} Widget with id/label: {new_id} already exists. Please give a unique explicit ID.", "Error:".red());
		}
		
		if !self.widgets.contains_key(&new_id) {
			let w = Button::new(label.clone());
			self.widgets.insert(new_id.clone(), Box::new(w));
		}
		self.frame_ids.insert(new_id.clone());
		
		// UPDATE STATE
		let b: &mut Button = self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap();
		b.value = label;
		b
	}
	
	pub fn checkbox(&mut self, id: WidgetId, label: String, value: bool) -> &mut Checkbox {
		let mut new_id = String::from("Checkbox:");
		
		new_id.push_str(match id {
			WidgetId::Auto => label.clone(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("{} Widget with id/label: {new_id} already exists. Please give a unique explicit ID.", "Error:".red());
		}
		
		if !self.widgets.contains_key(&new_id) {
			let w = Checkbox::new(label.clone(), value);
			self.widgets.insert(new_id.clone(), Box::new(w));
		}
		self.frame_ids.insert(new_id.clone());
		
		// UPDATE STATE
		let b: &mut Checkbox = self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap();
		b.text = label;
		b
	}
	
	pub async fn image(&mut self, id: WidgetId, path: String, size: Vec2) -> &mut ImageWidget {
		let mut new_id = String::from("Image:");
		
		new_id.push_str(match id {
			WidgetId::Auto => path.clone(),
			WidgetId::Explicit(s) => s,
		}.as_str());
		
		if self.frame_ids.contains(&new_id) {
			panic!("Widget with id/label: {new_id} already exists. Please give a unique explicit ID.");
		}
		
		let w = ImageWidget::new(path, size).await;
		self.widgets.insert(new_id.clone(), Box::new(w));
		self.frame_ids.insert(new_id.clone());
		
		self.widgets.get_mut(&new_id).unwrap().as_any_mut().downcast_mut().unwrap()
	}
}