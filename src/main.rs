use creeperUI::*;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;

    loop {
        ui.begin("login window").scope(|win| {
            win.text_colored("Important options below choose wisely:", YELLOW);
            
            win.same_line((), |win| {
                win.button("Start").set_background(DARKGREEN);
                win.button("Pause");
                win.button("Quit").set_background(MAROON);
            });
        });

        ui.draw();
        next_frame().await
    }
}
