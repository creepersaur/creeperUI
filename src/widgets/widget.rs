use std::any::Any;
use std::fmt::Display;
use macroquad::prelude::*;

pub trait Widget {
	fn as_any(&self)        -> &dyn Any;
	fn as_any_mut(&mut self)-> &mut dyn Any;
	fn render(&self, rect: &Rect, font: &Font) -> Option<Vec2>;
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) -> Option<Vec2>;
}