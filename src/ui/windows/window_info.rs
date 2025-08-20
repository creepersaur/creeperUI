use macroquad::prelude::*;

#[derive(Default)]
pub struct WindowInfo {
    pub close_button_rect: Rect,
    pub close_button_hovered: bool,
    pub close_button_pressed: bool,
    pub close_color: Color,
    pub ran_once: bool,
    pub min_size: Vec2,
    pub show_titlebar: bool,
    pub draggable: bool,
    pub resizable: bool,
    pub closable: bool,
}

impl WindowInfo {
    pub fn new() -> WindowInfo {
        WindowInfo {
            show_titlebar: true,
            draggable: true,
            resizable: true,
            closable: true,

            ..Default::default()
        }
    }
}
