use creeperUI::{ActionType, ProgressInfo, SliderInfo, Window, UI};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/Inter.ttf")).await;
    let mut checked = false;
    let mut x = 5;
    let mut text_value = String::new();

    let font = load_ttf_font("src/bauhs.ttf").await.unwrap();

    loop {
        let win = ui.begin("id").set_title("login window");
        
        win.text_ex("Login", WHITE, 30, None);
        
        win.separator().color = WHITE.with_alpha(0.0);

        win.text("Username:");
        text_value = win.textbox((), text_value).value.clone();
        
        win.text("Password:");
        text_value = win.textbox((), text_value).value.clone();

        win.separator();
        win.button((), "Login");

        ui.draw();
        // println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}

async fn test_window(win: &mut Window, checked: &mut bool, x: &mut usize) {
    win.set_size(vec2(400., 400.), ActionType::Once);

    if win.tabs((), vec!["Tab 1", "Tab 2"], 0).value == 1 {
        win.text("Bros on Tab 2 lmao");
        return;
    }

    win.separator().color = WHITE.with_alpha(0.0);

    win.text_ex("Documentation", WHITE, 30, None);

    win.separator().color = WHITE.with_alpha(0.0);

    win.text("Hello World");
    win.button((), "Hello World");

    win.separator();

    win.dropdown(
        (),
        (1..=*x).map(|x| format!("Option {x}")).collect(),
        "Option 1",
    );

    *checked = win
        .checkbox((), format!("Checked: {checked}"), *checked)
        .value;

    win.separator();

    win.progress_bar(
        (),
        "Progress bar:",
        ProgressInfo::Float {
            min: 0.0,
            max: 100.0,
            default_value: 50.0,
        },
    )
    .value = (mouse_position().0 / screen_width()) as f64 * 100.0;

    *x = win
        .slider(
            (),
            "Slider Int:",
            SliderInfo::Int {
                min: 1,
                max: 15,
                default_value: 5,
            },
        )
        .value as usize;

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

    win.image(
        (),
        format!("src/{}.png", pic),
        match pic.as_str() {
            "ten" => Some(vec2(60., 166.)),
            _ => None,
        },
    )
    .await;
}
