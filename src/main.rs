use creeperUI::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("login window").scope(|win| {
            win.button((), "Sigma");
            win.radio_buttons((), vec![
                "Hello",
                "World"
            ], "Hello");
            win.button((), "Sigma");
        });

        ui.draw();
        next_frame().await
    }
}
