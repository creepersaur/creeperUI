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
        
        let win = ui.begin("bald")
            .set_title("Bald Window")
            .set_titlebar(false);
        
        win.text((), "Lol theres no titlebar");
        win.text(generate_id!(), "Hello World");
        win.button((), "Button");
        
        let win = ui.begin("not bald")
            .set_title("Not bald");
        
        win.text((), "I at least have a titlebar");
        win.button((), "Button");
        
        ui.update();
        ui.render();
        ui.queue_removable();
        
        next_frame().await
    }
}
