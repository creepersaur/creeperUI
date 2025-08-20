#![allow(unused)]
use macroquad::prelude::*;
use creeperUI::{UI, Window, WindowHandler, ActionType, ProgressInfo, SliderInfo};

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new().await;
    let mut checked = false;
    let mut x = 5;
    
    loop {
        let win = ui.begin("id");
        test_window(win, &mut checked, &mut x).await;
        
        ui.draw();
        
        println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}

async fn test_window(win: &mut Window, checked: &mut bool, x: &mut usize) {
    win.set_size(vec2(400., 400.), ActionType::Once);
    win.text("Hello World");
    win.button((), "Hello World");
    
    win.separator();
    
    win.dropdown(
        (),
        (1..=*x).map(|x| format!("Option {x}")).collect(),
        "Option 1",
    );
    *checked = win.checkbox((), format!("Checked: {checked}"), *checked).value;
    
    win.separator();
    
    win.progress_bar((), "Progress bar:", ProgressInfo::Float {
        min: 0.0,
        max: 100.0,
        default_value: 50.0,
    }).value = (mouse_position().0 / screen_width()) as f64 * 100.0;
    
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