use macroquad::prelude::*;
use std::fs;

#[derive(Clone)]
pub struct WindowTheme {
    pub font: Option<Font>,
    pub title_thickness: f32,

    pub background: Color,
    pub inactive_titlebar: Color,
    pub active_titlebar: Color,
    pub win_stroke: Color,
    pub active_stroke: Color,
    pub hover_stroke: Color,
    pub close_button: Color,
    pub close_button_hover: Color,
    pub close_button_press: Color,
    pub resize_handle: Color,
}

impl WindowTheme {
    pub async fn new(font_path: Option<&str>) -> WindowTheme {
        WindowTheme {
            font: match font_path {
                Some(font_path) => match fs::exists(font_path).unwrap() {
                    true => Some(load_ttf_font(font_path).await.unwrap()),
                    _ => None,
                },
                _ => None
            },
            title_thickness: 30.0,

            background: Color::new(0.1, 0.1, 0.1, 1.0),
            active_titlebar: Color::new(0.2, 0.4, 0.7, 1.0),
            inactive_titlebar: Color::new(0.1, 0.3, 0.5, 1.0),
            win_stroke: Color::new(1.0, 1.0, 1.0, 0.3),
            active_stroke: Color::new(1.0, 1.0, 1.0, 0.7),
            hover_stroke: Color::new(1.0, 1.0, 1.0, 0.4),
            close_button: Color::new(0.0, 0.0, 0.0, 0.3),
            close_button_hover: RED,
            close_button_press: Color::new(0.7, 0.1, 0.1, 1.0),
            resize_handle: Color::new(0.3, 0.5, 0.7, 1.0),
        }
    }
}
