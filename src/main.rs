#![allow(unused)]
mod ui;
mod widgets;

use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use ui::windows::window_handler::WindowHandler;
use widgets::{widget::*, WidgetId};
use crate::ui::windows::action_type::ActionType;
use crate::widgets::SliderInfo;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = WindowHandler::new().await;
    let mut checked = false;
    let mut x = 5;
    
    loop {
        set_mouse_cursor(CursorIcon::Default);
        
        let win = ui.begin("win");
        win.text((), "Hello World");
        win.button((), "Hello World");
        
        win.separator();
        win.text((), "(I added separators ^^^)");
        
        win.separator()
            .set_color(RED)
            .set_thickness(15.0);
        win.text((), "(You can even change properties)");
        
        win.dropdown((), (1..=x).map(|x| format!("Option {x}")).collect(), "Option 1");
        checked = win.checkbox((), format!("Checked: {checked}"), true).value;
        
        x = win.slider((), "Slider Int:", SliderInfo::Int {
            min: 1,
            max: 15,
            default_value: 5
        }).value as usize;
        
        win.slider((), "Slider Float:", SliderInfo::Float {
            min: 5.0,
            max: 15.0,
            default_value: 5.0
        });
        
        let pic = win.dropdown((), vec!["ten_point", "ten"], "ten_point").value.clone();
        win.image((), format!("src/{}.png", pic), vec2(83., 131.)).await;
        
        ui.end_windows();
        
        next_frame().await
    }
}
