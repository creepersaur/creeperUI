#![allow(unused)]
mod ui;
mod widgets;

use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use ui::windows::window_handler::WindowHandler;
use widgets::{widget::*, WidgetId};
use crate::ui::windows::action_type::ActionType;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = WindowHandler::new().await;
    let mut popup = false;
    
    loop {
        set_mouse_cursor(CursorIcon::Default);
        
        let main = ui.begin("main");
        main.button((), "Hello World");
        main.checkbox(generate_id!(), "Hello World", true);
        main.image((), "src/ten_point.png", vec2(300., 300.)).await;
        main.text((), "Hello World");
        
        ui.end_windows();
        
        next_frame().await
    }
}
