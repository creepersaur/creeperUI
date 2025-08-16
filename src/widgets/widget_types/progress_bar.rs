use std::any::Any;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget::Widget;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

pub struct ProgressBar {
	pub text: String,
	pub value: f64,
	pub info: ProgressInfo,
	pub hovered: bool,
	pub pressed: bool,
	value_thickness: f32,
}

#[derive(Clone)]
pub enum ProgressInfo {
	Float { min: f64, max: f64, default_value: f64 },
	Int { min: i32, max: i32, default_value: i32 }
}

impl ProgressBar {
	pub fn new(text: String, info: ProgressInfo) -> Self {
		Self {
			text,
			info: info.clone(),
			value: match info {
				ProgressInfo::Int { min, max, default_value } => default_value as f64,
				ProgressInfo::Float { min, max, default_value } => default_value
			},
			value_thickness: 15.0,
			hovered: false,
			pressed: false,
		}
	}
}

impl Widget for ProgressBar {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
	
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.text.to_string(),
			Some(info.font),
			13,
			1.0
		);
		
		for i in 0..4 {
			draw_text_ex(
				&self.text.to_string(),
				0.0,
				text_dim.height + info.rect.h + 5.0,
				TextParams {
					font: Some(info.font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		// BASE BAR
		let r_width = info.win_rect.w - text_dim.width - 15.0;
		draw_rectangle(
			text_dim.width + 5.0,
			info.rect.h + 5.0,
			r_width,
			text_dim.height + 4.0,
			Color::new(0.1,0.25,0.4, 1.0)
		);
		
		// VALUE
		let (slider_type, min, max) = match self.info {
			ProgressInfo::Float { min, max, default_value } => ("float", min, max),
			ProgressInfo::Int { min, max, default_value } => ("int", min as f64, max as f64)
		};
		
		draw_rectangle(
			text_dim.width + 5.0,
			info.rect.h + 5.0,
			r_width * (self.value / max) as f32,
			text_dim.height + 4.0,
			Color::new(0.34, 0.54, 0.8, 1.0)
		);
		
		//PRINT VALUE
		
		for i in 0..14 {
			draw_text_ex(
				&match slider_type {
					"float" => format!("{:.2}", self.value),
					"int" => (self.value as i32).to_string(),
					_ => String::new()
				},
				text_dim.width + 15.0,
				text_dim.height + info.rect.h + 5.0,
				TextParams {
					font: Some(info.font),
					font_size: 13,
					color: WHITE,
					..Default::default()
				}
			);
		}
		
		Some(vec2(text_dim.width + r_width, text_dim.height + 10.0))
	}
	
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
		let text_dim = measure_text(
			&self.text.to_string(),
			Some(info.font),
			13,
			1.0
		);
		
		let (slider_type, min, max) = match self.info {
			ProgressInfo::Float { min, max, default_value } => ("float", min, max),
			ProgressInfo::Int { min, max, default_value } => ("int", min as f64, max as f64)
		};
		
		self.value = self.value.clamp(min, max);
		
		if slider_type == "int" {
			self.value = self.value.round()
		}
		
		Some(vec2(text_dim.width, text_dim.height + 10.0))
	}
}