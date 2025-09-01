use macroquad::math::Vec2;

pub struct WindowProperties {
    pub title: Option<String>,
    pub position: Option<Vec2>,
    pub size: Option<Vec2>,
    pub draggable: bool,
    pub resizable: bool,
    pub closable: bool,
    pub scrollable: bool,
}

impl Default for WindowProperties {
    fn default() -> Self {
        Self {
            title: None,
            position: None,
            size: None,
            draggable: true,
            resizable: true,
            closable: true,
            scrollable: true,
        }
    }
}
