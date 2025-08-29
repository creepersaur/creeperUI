use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use macroquad::prelude::*;
use std::any::Any;

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn render(&self, info: &mut RenderInfo) -> Option<Vec2>;
    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2>;
}
