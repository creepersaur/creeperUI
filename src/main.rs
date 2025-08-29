use creeperUI::{ActionType, ProgressInfo, SliderInfo, Window, UI};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/Inter.ttf")).await;
    let mut checked = false;
    let mut x = 5;
    
    let font = load_ttf_font("src/bauhs.ttf").await.unwrap();

    loop {
        let win = ui
            .begin("id")
            .scope_async(async |win| {
                let tab = win.tabs((), vec!["Option1", "Option2", "Option3"], 0).value;

                // Option1
                win.scope_if(tab == 0, |win| {
                    if win.button((), "Press me").pressed {
                        win.text("JUMP SCARE");
                    }
                    win.text(format!("FPS: {}", get_fps()));
                });

                // Option2
                win.scope_if(tab == 1, |win| {
                    win.text_ex("Hello", RED, 30, Some(font.clone()));

                    win.text("This is option 2");
                    win.button((), "Hello World");
                });

                // Option3
                win.scope_async_if(tab == 2, async |win| {
                    win.image((), "src/job_app.png", Some(vec2(290.0, 400.0)))
                        .await;
                })
                .await;
            })
            .await;

        test_window(win, &mut checked, &mut x).await;

        if !ui.taken && is_mouse_button_pressed(MouseButton::Left) {
            println!("Yes: {}", get_frame_time() * 1000.0);
        }

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
