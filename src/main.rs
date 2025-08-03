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
    let mut check = true;
    
    loop {
        set_mouse_cursor(CursorIcon::Default);
        
        let win = ui.begin("bald")
            .set_title("Bald Window")
            .set_titlebar(false)
            .set_pos(vec2(200., 200.), ActionType::Once);
        
        win.text((), "Lol theres no titlebar");
        win.text(generate_id!(), "Hello World");
        win.button((), "Button");
        
        if check {
            win.text((), "Checked!");
        }
        
        check = win.checkbox(generate_id!(), "IsChecked", check).value;
        
        if check {
            win.button((), "Wow you checked it.");
        }
        
        let win = ui.begin("not bald")
            .set_title("Not bald");
        
        win.text((), "I have a titlebar");
        win.button((), "Button");
        
        ui.update();
        ui.render();
        ui.queue_removable();
        
        next_frame().await
    }
}
