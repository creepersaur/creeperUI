#![allow(unused)]
mod ui;
mod widgets;

use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use ui::windows::window_handler::WindowHandler;
use widgets::{widget::*, WidgetId};

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = WindowHandler::new().await;
    
    loop {
        set_mouse_cursor(CursorIcon::Default);
        
        let win = ui.begin("hello")
            .set_title("Hello");
        
        win.text(line_id!(), &"Wow this works");
        if win.button(123, &"CLICK ME!").clicked {
            println!("Grr you clicked the first button!")
        }
        win.text((), &"Wow this works");
        
        ui.update();
        ui.render();
        ui.queue_removable();
        
        next_frame().await
    }
}
