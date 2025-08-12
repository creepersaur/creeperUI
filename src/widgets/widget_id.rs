use std::fmt::{Display, Formatter};

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

impl Display for WidgetId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			WidgetId::Auto => "Auto".into(),
			WidgetId::Explicit(s) => s.clone()
		})
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
macro_rules! gen_id {
    () => {
		format!(
            "{}:{}:{}",
			file!(),
            line!(),
            column!()
        )
	};
	($extra_label: expr) => {
		format!(
            "{}:{}:{}:{}",
            $extra_label,
			file!(),
            line!(),
            column!()
        )
	};
}

impl_widget_id!(String, &str, i32, usize, u32, f32, f64, bool, char);
