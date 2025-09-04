use crate::widgets::widget_holder::{RenderInfo, UpdateInfo};
use crate::widgets::Widget;
use macroquad::prelude::*;
use std::any::Any;

pub struct Separator {
    pub thickness: f32,
    pub padding: f32,
    pub color: Color,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            thickness: 2.0,
            padding: 5.0,
            color: Color::new(1.0, 1.0, 1.0, 0.65),
        }
    }

    pub fn set_thickness(&mut self, thickness: f32) -> &mut Self {
        self.thickness = thickness;
        self
    }

    pub fn set_padding(&mut self, padding: f32) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
}

impl Widget for Separator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn render(&self, info: &mut RenderInfo) -> Option<Vec2> {
        if info.same_line {
            draw_line(
                info.rect.x + self.padding + (self.thickness / 4.0).floor(),
                info.rect.y,
                info.rect.x + self.padding + (self.thickness / 4.0).floor(),
                info.rect.y + 10.0 + info.rect.h,
                self.thickness,
                self.color,
            );
            
            Some(vec2(self.padding * 2.0, 0.0))
        } else {
            draw_line(
                5.0,
                info.rect.y + info.rect.h + self.padding + (self.thickness / 4.0).floor(),
                info.win_rect.w - 15.0,
                info.rect.y + info.rect.h + self.padding + (self.thickness / 4.0).floor(),
                self.thickness,
                self.color,
            );
            
            Some(vec2(info.rect.w, self.padding * 2.0))
        }
    }

    fn update(&mut self, info: &mut UpdateInfo) -> Option<Vec2> {
        Some(vec2(info.rect.w, self.padding * 2.0))
    }
}
