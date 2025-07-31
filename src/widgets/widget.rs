use std::any::Any;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub enum WidgetId {
	Auto,
	Explicit(String)
}

impl From<()> for WidgetId {
	fn from(_: ()) -> Self {
		WidgetId::Auto
	}
}

macro_rules! impl_widget_id {
    ($($t:ty),*) => {
        $(
            impl From<$t> for WidgetId {
                fn from(value: $t) -> Self {
                    WidgetId::Explicit(value.to_string())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! generate_id {
    () => {
		$crate::widgets::widget::WidgetId::Explicit(format!(
            "{}:{}:{}",
			file!(),
            line!(),
            column!()
        ))
	};
	($extra_label: expr) => {
		$crate::widgets::widget::WidgetId::Explicit(format!(
            "{}:{}:{}:{}",
            $extra_label,
			file!(),
            line!(),
            column!()
        ))
	};
}

impl_widget_id!(String, &str, i32, usize, u32, f32, f64, bool, char);

pub trait Widget {
	fn as_any(&self)        -> &dyn Any;
	fn as_any_mut(&mut self)-> &mut dyn Any;
	fn render(&self, rect: &Rect, font: &Font) -> Option<Vec2>;
	fn update(&mut self, rect: &Rect, hover: bool, mouse: Vec2, font: &Font) -> Option<Vec2>;
}