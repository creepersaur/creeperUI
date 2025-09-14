use creeperUI::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("login window").scope(|win| {
            win.same_line((), |win| {
                win.button((), "hello");
            });
            win.button((), "hello");
        });

        ui.draw();
        next_frame().await
    }
}
