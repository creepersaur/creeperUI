use std::any::Any;
use std::fmt::Display;
use macroquad::prelude::*;
use crate::ui::mouse_action::WidgetAction;
use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};

pub trait Widget {
	fn as_any(&self)        -> &dyn Any;
	fn as_any_mut(&mut self)-> &mut dyn Any;
	fn render(&self, info: &mut RenderInfo) -> Option<Vec2>;
	fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2>;
}