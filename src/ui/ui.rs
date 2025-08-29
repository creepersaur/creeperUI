use crate::ui::windows::window::Window;
use crate::ui::windows::window_handler::WindowHandler;

pub struct UI {
    pub taken: bool,
    handler: WindowHandler,
}

impl UI {
    pub async fn new(font_path: Option<&str>) -> Self {
        Self {
            handler: WindowHandler::new(font_path).await,
            taken: false,
        }
    }

    pub fn begin(&mut self, id: impl ToString) -> &mut Window {
        self.handler.begin(id)
    }

    pub fn draw(&mut self) -> bool {
        let taken = self.handler.start_windows();
        self.taken = taken;
        self.handler.end_windows();

        taken
    }
}
