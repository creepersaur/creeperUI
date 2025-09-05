use creeperUI::{gen_id, ActionType, ProgressInfo, SliderInfo, Window, UI};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("1").set_title("login window").scope(|win| {
            win.same_line((), |win| {
                win.dropdown((), vec![
                    "Hello",
                    "World"
                ], "Hello");
                
                win.text("Camera Type:");
                
                win.dropdown((), vec![
                    "Custom",
                    "Scriptable",
                    "Orbit",
                    "Fixed",
                ], "Custom");
            });
        });

        ui.draw();
        // println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}