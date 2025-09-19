use creeperUI::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("login window").scope(|win| {
            win.text_colored("Important options below choose wisely:", YELLOW);
            win.radio_buttons((), vec![
                "Easy",
                "Medium",
                "Hard",
                "Emotional Damage",
            ], "Easy");
            win.button("Sigma");
        });

        ui.draw();
        next_frame().await
    }
}
