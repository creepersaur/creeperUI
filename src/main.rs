#![allow(unused)]
mod ui;
mod widgets;

use crate::ui::windows::action_type::ActionType;
use crate::widgets::{ProgressInfo, SliderInfo};
use macroquad::prelude::*;
use ui::windows::window_handler::WindowHandler;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = WindowHandler::new().await;
    let mut checked = false;
    let mut x = 5;
    
    loop {
        // test_window(&mut ui, &mut checked, &mut x).await;
        
        let win = ui.begin("win").set_size(vec2(400.0, 200.0), ActionType::Once);
        let tabs = win.tabs((), vec!["hello", "world", "foo", "bar"], 0);
        
        match tabs.value {
            0 => {
                win.text("This is tab one");
            }
            1 => {
                win.text("This is tab 2");
                win.text("Here, have a");
                win.button((), "Button");
            }
            2 => {
                win.text("It's TV TIME!");
                win.image((), "src/ten_point.png", None).await;
            }
            
            _ => {
                win.text("Theres.... Nothing here :(");
            }
        }
        
        ui.start_windows();
        ui.end_windows();
        
        println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
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