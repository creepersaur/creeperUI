use creeperUI::{gen_id, ActionType, ProgressInfo, SliderInfo, Window, UI};
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut ui = UI::new(Some("src/arial.ttf")).await;
    
    let mut notes = vec![];

    loop {
        ui.begin("1").set_title("login window").scope(|win| {
            win.text_ex("To-do List", WHITE, 20, None);
            win.separator().set_color(BLACK.with_alpha(0.0)).set_padding(1.0);
            
            win.same_line("hello", |win| {
                let text = win.textbox((), "Do something....").value.clone();
                if win.button((), "Add").clicked {
                    notes.push(text);
                }
            });
            
            win.checkbox(gen_id!(), "Make todo?", false);
            win.separator();
            
            for (i, v) in notes.iter().enumerate() {
                win.same_line(gen_id!(i), |win| {
                    win.text(format!(" - {v}"));
                    // win.button((), "Add");
                    win.checkbox(gen_id!(i), "", false);
                });
            }
        });

        ui.draw();
        // println!("Frame Time: {:2}ms", get_frame_time() * 1000.0);
        next_frame().await
    }
}