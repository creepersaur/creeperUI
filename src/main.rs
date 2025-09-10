use creeperUI::{gen_id, ActionType, ProgressInfo, SliderInfo, Window, UI};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("login window").scope(|win| {
            if **win.button((), "Hello") {
                println!("Pressed");
            }
        });

        ui.draw();
        // println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}