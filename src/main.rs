mod ui;

use macroquad::miniquad::CursorIcon;
use macroquad::miniquad::window::set_mouse_cursor;
use macroquad::prelude::*;
use crate::ui::windows::window_handler::WindowHandler;

#[macroquad::main("Hello")]
async fn main() {
    let mut winhandler = WindowHandler::new().await;
    let mut open = true;
    
    loop {
        set_mouse_cursor(CursorIcon::Default);
        
        winhandler.begin("hello")
            .set_title("First")
            .once(|w| {
                w.set_pos(vec2(200., 200.));
            });
        
        winhandler.begin("world")
            .set_title("Second");
        
        winhandler.update();
        winhandler.render();
        winhandler.queue_removable();
        
        next_frame().await
    }
}
