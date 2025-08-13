// #![allow(unused)]
mod ui;
mod widgets;

use crate::ui::windows::action_type::ActionType;
use crate::widgets::SliderInfo;
use macroquad::prelude::*;
use ui::windows::window_handler::WindowHandler;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = WindowHandler::new().await;
    let mut checked = false;
    let mut x = 5;

    loop {
        // test_window(&mut ui, &mut checked, &mut x).await;
        
        if ui.begin("").button((), "Hello").clicked {
            println!("Yes");
        };
        
        ui.start_windows();
        ui.end_windows();
        
        // println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}


async fn test_window(ui: &mut WindowHandler, checked: &mut bool, x: &mut usize) {
    let win = ui.begin("win").set_size(vec2(400., 400.), ActionType::Once);
    win.text("Hello World");
    win.button((), "Hello World");
    
    win.separator();
    
    win.dropdown(
        (),
        (1..=*x).map(|x| format!("Option {x}")).collect(),
        "Option 1",
    );
    *checked = win.checkbox((), format!("Checked: {checked}"), true).value;
    
    *x = win.slider(
        (),
        "Slider Int:",
        SliderInfo::Int {
            min: 1,
            max: 15,
            default_value: 5,
        },
    ).value as usize;
    
    win.slider(
        (),
        "Slider Float:",
        SliderInfo::Float {
            min: 5.0,
            max: 15.0,
            default_value: 5.0,
        },
    );
    
    let pic = win
        .dropdown((), vec!["ten_point", "ten"], "ten_point")
        .value
        .clone();
    
    win.image((), format!("src/{}.png", pic), match pic.as_str() {
        "ten" => Some(vec2(60., 166.)),
        _ => None,
    }).await;
}