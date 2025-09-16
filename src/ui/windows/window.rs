use crate::text_ex::TextEx;
use crate::ui::mouse_action::{MouseAction, WidgetAction};
use crate::ui::windows::win_resize_handles::WindowResizeHandles;
use crate::ui::windows::window_info::WindowInfo;
use crate::widget_holder::{GlobalRenderTargets, WidgetHolder};
use crate::widgets::*;
use crate::{ActionType, WindowId, WindowProperties, WindowTheme};
use indexmap::IndexSet;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::misc::rounded_rect::{draw_rounded_rect, draw_rounded_rect_ex, draw_rounded_rect_stroke};

pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub rect: Rect,
    pub widget_holders: HashMap<String, WidgetHolder>, // Changed to HashMap
    pub holder_ids: IndexSet<String>,                  // Retained for order
    pub info: WindowInfo,
    pub taken: bool,
    theme: WindowTheme,
    resize_handles: WindowResizeHandles,
    pub scroll_y: f32,
    pub max_scroll_y: f32,
    pub scrolling: bool,
    render_targets: GlobalRenderTargets,

    mouse: Vec2,
    pub open: bool,
    pub active: bool,
    pub hover: bool,
    pub mouse_action: WidgetAction,
    pub dragging: Option<Vec2>,
    pub resizing: bool,
}

impl Window {
    pub fn new(id: WindowId, name: String, theme: WindowTheme) -> Window {
        Window {
            theme,
            id,
            taken: false,
            title: name,
            rect: Rect::new(0.0, 0.0, 200.0, 150.0),
            info: WindowInfo::new(),
            resize_handles: WindowResizeHandles::new(),
            widget_holders: HashMap::from([(String::from("__Main__"), WidgetHolder::new(false))]),
            holder_ids: IndexSet::from([String::from("__Main__")]),
            scroll_y: 0.0,
            max_scroll_y: 0.0,
            scrolling: false,
            render_targets: GlobalRenderTargets::default(),

            open: true,
            mouse: mouse_position().into(),
            active: true,
            hover: false,
            dragging: None,
            resizing: false,
            mouse_action: WidgetAction::new(),
        }
    }

    pub fn set_title(&mut self, title: impl ToString) -> &mut Window {
        self.title = title.to_string();
        self
    }

    pub fn set_titlebar(&mut self, showing: bool) -> &mut Window {
        self.info.show_titlebar = showing;
        self
    }

    pub fn set_pos(
        &mut self,
        position: Vec2,
        action_type: impl Into<ActionType> + Clone,
    ) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.x = position.x;
            self.rect.y = position.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.x = position.x;
            self.rect.y = position.y;
        }

        self
    }

    pub fn set_size(
        &mut self,
        size: Vec2,
        action_type: impl Into<ActionType> + Clone,
    ) -> &mut Window {
        if action_type.clone().into() == ActionType::Once && !self.info.ran_once {
            self.rect.w = size.x;
            self.rect.h = size.y;
        } else if action_type.into() == ActionType::EachFrame {
            self.rect.w = size.x;
            self.rect.h = size.y;
        }

        self
    }

    pub fn set_properties(&mut self, properties: WindowProperties) -> &mut Window {
        self.info.draggable = properties.draggable;
        self.info.resizable = properties.resizable;
        self.info.closable = properties.closable;
        self.info.scrollable = properties.scrollable;
        if let Some(title) = properties.title {
            self.title = title;
        }
        if let Some(position) = properties.position {
            self.set_pos(position, ActionType::EachFrame);
        }
        if let Some(size) = properties.size {
            self.set_size(size, ActionType::EachFrame);
        }

        self
    }

    pub fn set_draggable(&mut self, draggable: bool) -> &mut Window {
        self.info.draggable = draggable;
        self
    }

    pub fn set_resizable(&mut self, resizable: bool) -> &mut Window {
        self.info.resizable = resizable;
        self
    }

    pub fn set_closable(&mut self, closable: bool) -> &mut Window {
        self.info.closable = closable;
        self
    }

    pub fn set_scrollable(&mut self, scrollable: bool) -> &mut Window {
        self.info.scrollable = scrollable;
        self
    }

    pub fn set_active(&mut self, active: bool) -> &mut Window {
        self.active = active;
        self
    }

    pub fn close(&mut self) -> &mut Window {
        self.open = false;
        self
    }

    pub fn show(&mut self) {
        self.open = true;
    }

    pub fn once(&mut self, f: impl FnOnce(&mut Window)) -> &mut Window {
        if !self.info.ran_once {
            f(self);
        }
        self
    }

    pub fn set_min_size(&mut self, size: Vec2) -> &mut Window {
        self.info.min_size = size;
        self
    }
}

/////////////////////////////////////
// RENDER
/////////////////////////////////////

impl Window {
    pub fn render(&self) {
        // BASE
        // draw_rectangle(
        //     self.rect.x,
        //     self.rect.y,
        //     self.rect.w,
        //     self.rect.h,
        //     self.theme.background,
        // );
        
        draw_rounded_rect_stroke(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.theme.border_radius,
            1.0,
            match (self.active, self.hover) {
                (false, true) => self.theme.hover_stroke,
                (true, _) => self.theme.active_stroke,
                _ => self.theme.win_stroke,
            },
            self.theme.background,
        );

        self.start_widget_render();

        self.draw_scrollbar();

        // TITLEBAR
        if self.info.show_titlebar {
            self.draw_titlebar();
        }

        if !self.info.scroll_hovered && !self.info.scroll_pressed.is_some() {
            self.draw_resize_handles();
        }
        
        if let Some(target) = &self.render_targets.target_3 {
            draw_texture_ex(
                &target.texture,
                self.rect.x + 5.0,
                self.rect.y + self.theme.title_thickness,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }
    }

    pub fn start_widget_render(&self) {
        let scale = 0.01;
        
        let title_thickness = match self.info.show_titlebar {
            false => 0.0,
            _ => 30.0,
        };
        let (zoom_x, zoom_y) = (
            scale / self.rect.w * 200.0,
            scale / (self.rect.h - title_thickness) * 200.0,
        );
        
        let cam_1 = &Camera2D {
            zoom: vec2(zoom_x, zoom_y),
            target: vec2(1.0 / zoom_x, 1.0 / zoom_y),
            render_target: Some(self.render_targets.target_1.clone().unwrap()),
            ..Default::default()
        };
        let cam_2 = &Camera2D {
            zoom: vec2(zoom_x, zoom_y),
            target: vec2(1.0 / zoom_x, 1.0 / zoom_y),
            render_target: Some(self.render_targets.target_2.clone().unwrap()),
            ..Default::default()
        };
        let cam_3 = &Camera2D {
            zoom: vec2(
                scale / screen_width() * 200.0,
                scale / screen_height() * 200.0,
            ),
            target: vec2(
                1.0 / scale * screen_width() / 200.0,
                1.0 / scale * screen_height() / 200.0,
            ),
            render_target: Some(self.render_targets.target_3.clone().unwrap()),
            ..Default::default()
        };

        set_camera(cam_3);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        set_camera(cam_2);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        set_camera(cam_1);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        // DRAW TOP-LAYER OF WIDGETS
        let new_rect = self.rect.clone();
        let mut vertical_offset = 0.0;

        for i in self.holder_ids.iter() {
            set_camera(cam_1);
            let holder = self.widget_holders.get(i).unwrap();

            let rect_h = holder.render(
                &new_rect,
                self.scroll_y,
                &self.theme.font,
                vertical_offset,
                [&cam_1, &cam_2, &cam_3],
            );
            
            vertical_offset += rect_h + self.theme.holder_padding;
        }
        
        set_default_camera();

        if let Some(target) = &self.render_targets.target_1 {
            draw_texture_ex(
                &target.texture,
                self.rect.x + 5.0,
                self.rect.y + title_thickness,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        0.0,
                        0.0,
                        (self.rect.w - 5.0).max(0.0),
                        (self.rect.h - 5.0 - title_thickness).max(0.0),
                    )),
                    ..Default::default()
                },
            )
        }

        if let Some(target) = &self.render_targets.target_2 {
            draw_texture_ex(
                &target.texture,
                self.rect.x + 5.0,
                self.rect.y + title_thickness,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        0.0,
                        0.0,
                        (self.rect.w - 5.0).max(0.0),
                        (self.rect.h - 5.0 - title_thickness).max(0.0),
                    )),
                    ..Default::default()
                },
            )
        }
    }

    pub fn draw_titlebar(&self) {
        draw_rounded_rect_ex(
            self.rect.x + 1.0,
            self.rect.y + 1.0,
            self.rect.w - 2.0,
            self.theme.title_thickness - 1.0,
            self.theme.border_radius,
            self.theme.border_radius,
            0.0,
            0.0,
            match self.active {
                true => self.theme.active_titlebar,
                _ => self.theme.inactive_titlebar,
            },
        );

        draw_text_ex(
            &self.title,
            self.rect.x + 10.0,
            self.rect.y + self.theme.title_thickness - 10.0,
            TextParams {
                font: match &self.theme.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                font_size: 13,
                ..Default::default()
            },
        );

        if self.info.closable {
            self.draw_close_button();
        }
    }

    pub fn draw_close_button(&self) {
        draw_rounded_rect_ex(
            self.rect.x + self.rect.w - self.theme.title_thickness,
            self.rect.y + 1.0,
            self.theme.title_thickness - 1.0,
            self.theme.title_thickness - 1.0,
            0.0,
            self.theme.border_radius,
            0.0,
            0.0,
            self.info.close_color,
        );

        draw_text_ex(
            "x",
            self.rect.x + self.rect.w - self.theme.title_thickness
                + self.theme.title_thickness / 2.0
                - 5.0,
            self.rect.y + self.theme.title_thickness - 10.0,
            TextParams {
                font: match &self.theme.font {
                    Some(f) => Some(&f),
                    _ => None,
                },
                font_size: 14,
                ..Default::default()
            },
        );
    }

    pub fn draw_resize_handles(&self) {
        self.resize_handles.render(&self.rect, &self.theme);
    }

    pub fn draw_scrollbar(&self) {
        if self.max_scroll_y < 5.0 {
            return;
        }

        let viewport_h = self.rect.h - self.theme.title_thickness;
        let content_h = viewport_h + self.max_scroll_y;
        let thickness = self.theme.scrollbar_thickness;

        let thumb_h = if content_h <= 0.0 {
            viewport_h
        } else {
            (viewport_h / content_h) * viewport_h
        };

        let thumb_y = if self.max_scroll_y > 0.0 {
            self.rect.y
                + self.theme.title_thickness
                + (self.scroll_y / self.max_scroll_y) * (viewport_h - thumb_h)
        } else {
            self.rect.y + self.theme.title_thickness
        };

        draw_rectangle(
            self.rect.x + self.rect.w - thickness,
            thumb_y,
            thickness,
            thumb_h,
            match (self.info.scroll_hovered, self.info.scroll_pressed.is_some()) {
                (true, false) => WHITE.with_alpha(0.4),
                (_, true) => WHITE.with_alpha(0.6),
                _ => WHITE.with_alpha(0.2),
            },
        );
    }
}

/////////////////////////////////////
// UPDATE
/////////////////////////////////////

impl Window {
    pub fn update(&mut self, active_taken: bool, mouse_action: MouseAction) {
        if active_taken {
            self.active = false;
        }
        self.mouse = mouse_position().into();

        let hover = self.rect.contains(self.mouse);
        let window_action = mouse_action == MouseAction::WindowHover(self.id)
            || mouse_action == MouseAction::Normal;
        self.hover = window_action && hover;

        if is_mouse_button_pressed(Left) && window_action {
            if !active_taken {
                self.active = hover;
            } else {
                self.active = false;
            }
        }

        self.taken = false;
        let mut mouse_action = WidgetAction::new();
        let mut scroll_hov = false;
        if self.info.scroll_hovered {
            mouse_action.taken = true;
            scroll_hov = true;
        }
        
        let mut widget_action = WidgetAction::new();
        let mut vertical_offset = -self.theme.holder_padding;
        
        if window_action {
            let new_rect = self.rect.clone();
            
            for i in self.holder_ids.iter() {
                let holder = self.widget_holders.get_mut(i).unwrap();

                let (action, holder_rect) = holder.update(
                    &new_rect,
                    vertical_offset,
                    self.info.show_titlebar,
                    hover,
                    self.mouse,
                    self.scroll_y,
                    &self.theme.font,
                    &mut mouse_action,
                );
                
                widget_action = action;
                vertical_offset += holder_rect.h + self.theme.holder_padding;
            }
        }
        
        if widget_action.taken && !scroll_hov {
            self.taken = true;
        } else {
            self.handle_scrolling(vertical_offset);
        }

        if self.info.resizable {
            self.update_resize_handles(window_action, self.taken || self.scrolling);
        }
        if self.resizing || self.scrolling {
            self.taken = true;
        } else if self.info.closable {
            self.handle_close_button(window_action);
        }
        if self.resize_handles.hovering_handle.is_some() {
            self.taken = true;
        }

        if self.info.draggable {
            self.handle_dragging();
        }

        if let Some(start_offset) = self.dragging {
            self.set_pos(self.mouse + start_offset, ActionType::EachFrame);
            self.clamp();
        } else if self.resizing {
            self.clamp();
        }
        
        self.rect.x = self.rect.x.floor();
        self.rect.y = self.rect.y.floor();

        self.ensure_render_targets(match self.info.show_titlebar {
            false => 0.0,
            _ => self.theme.title_thickness,
        });
    }

    fn handle_dragging(&mut self) {
        let mut title_rect = self.rect.clone();
        title_rect.h = self.theme.title_thickness;

        if self.active
            && !self.resizing
            && !self.info.close_button_pressed
            && is_mouse_button_pressed(Left)
            && title_rect.contains(self.mouse)
        {
            self.dragging = Some(vec2(self.rect.x, self.rect.y) - self.mouse);
        }

        if is_mouse_button_released(Left) {
            self.dragging = None;
        }
    }

    pub fn clamp(&mut self) {
        // CLAMP SIZE

        if self.rect.h < self.theme.title_thickness + self.theme.border_radius / 2.0 - 1.0 {
            self.rect.h = self.theme.title_thickness + self.theme.border_radius / 2.0 - 1.0;
        }

        let title_dim = measure_text(
            &self.title,
            match &self.theme.font {
                Some(f) => Some(&f),
                _ => None,
            },
            14,
            1.0,
        );
        if self.info.show_titlebar {
            if self.rect.w < title_dim.width.max(self.info.close_button_rect.w - 10.0) + 20.0 {
                self.rect.w = title_dim.width.max(self.info.close_button_rect.w - 10.0) + 20.0
            }
        } else {
            self.rect.w = self.rect.w.max(self.theme.title_thickness);
            self.rect.h = self.rect.h.max(self.theme.title_thickness);
        }

        if self.rect.w < self.info.min_size.x {
            self.rect.w = self.info.min_size.x;
        }

        if self.rect.h < self.info.min_size.y {
            self.rect.h = self.info.min_size.y;
        }

        // CLAMP POSITION

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        } else if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }

        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        } else if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    fn handle_close_button(&mut self, window_action: bool) {
        if !self.info.show_titlebar {
            self.info.close_button_hovered = false;
            return;
        }

        self.info.close_button_rect.x = self.rect.x + self.rect.w - self.theme.title_thickness;
        self.info.close_button_rect.y = self.rect.y;
        self.info.close_button_rect.w = self.theme.title_thickness;
        self.info.close_button_rect.h = self.theme.title_thickness;

        if self.info.close_button_rect.contains(self.mouse) && window_action {
            self.info.close_button_hovered = true;
            if is_mouse_button_pressed(Left) && self.active {
                self.info.close_button_pressed = true;
            } else if is_mouse_button_released(Left) {
                if self.info.close_button_pressed {
                    self.close();
                }
            }
        } else {
            self.info.close_button_hovered = false;
        }

        if is_mouse_button_released(Left) {
            self.info.close_button_pressed = false;
        }

        self.info.close_color = Color::from_vec(self.info.close_color.to_vec().lerp(
            match (
                self.info.close_button_pressed,
                self.info.close_button_hovered,
            ) {
                (false, true) => self.theme.close_button_hover.to_vec(),
                (true, _) => self.theme.close_button_press.to_vec(),
                _ => self.theme.close_button.to_vec(),
            },
            0.2,
        ));
    }

    fn update_resize_handles(&mut self, window_action: bool, taken: bool) {
        if !self.resize_handles.resizing.is_some()
            && (self.info.scroll_pressed.is_some() || self.info.scroll_hovered)
        {
            self.resize_handles.resizing = None;
        } else {
            self.resize_handles
                .update(&mut self.rect, window_action, taken);
        }
        self.resizing = self.resize_handles.resizing.is_some();
    }

    fn handle_scrolling(&mut self, vertical_offset: f32) {
        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            self.scroll_y -= wheel.1;
        } else if wheel.0 != 0.0 {
            self.scroll_y += wheel.0;
        }
        self.max_scroll_y = (vertical_offset - self.rect.h + self.theme.title_thickness).max(0.0);

        //////////////////////////////////////////
        // SCROLL BAR
        //////////////////////////////////////////

        if is_mouse_button_released(Left) {
            self.info.scroll_pressed = None;
        }

        if self.max_scroll_y < 5.0 {
            return;
        }

        let viewport_h = self.rect.h - self.theme.title_thickness;
        let content_h = viewport_h + self.max_scroll_y;
        let thickness = self.theme.scrollbar_thickness;

        let thumb_h = if content_h <= 0.0 {
            viewport_h
        } else {
            (viewport_h / content_h) * viewport_h
        };

        let thumb_y = if self.max_scroll_y > 0.0 {
            self.rect.y
                + self.theme.title_thickness
                + (self.scroll_y / self.max_scroll_y) * (viewport_h - thumb_h)
        } else {
            self.rect.y + self.theme.title_thickness
        };

        let bar_rect = Rect::new(
            self.rect.x + self.rect.w - thickness,
            thumb_y,
            thickness,
            thumb_h,
        );

        if bar_rect.contains(self.mouse) {
            self.info.scroll_hovered = true;
            if is_mouse_button_pressed(Left) {
                self.info.scroll_pressed = Some(self.mouse.y);
            }
        } else {
            self.info.scroll_hovered = false;
        }

        if let Some(start_y) = self.info.scroll_pressed {
            self.scroll_y += (self.mouse.y - start_y) * self.max_scroll_y
                / (self.rect.h - self.theme.title_thickness)
                * 2.0;
            self.info.scroll_pressed = Some(self.mouse.y);
        }

        self.scroll_y = self.scroll_y.clamp(0.0, self.max_scroll_y);
    }

    pub fn ensure_render_targets(&mut self, title_thickness: f32) {
        let win_w = self.rect.w as u32;
        let win_h = (self.rect.h - title_thickness) as u32;
        let (screen_w, screen_h) = (screen_width() as u32, screen_height() as u32);

        // Check if screen size changed
        let needs_update = self
            .render_targets
            .target_1
            .as_ref()
            .map(|t| t.texture.width() as u32 != win_w || t.texture.height() as u32 != win_h)
            .unwrap_or(true);
        
        let update_3 = self.render_targets.target_3.as_ref()
            .map(|t| t.texture.width() as u32 != screen_w || t.texture.height() as u32 != screen_h)
            .unwrap_or(true);

        if needs_update {
            self.render_targets.target_1 = Some(render_target(win_w, win_h));
            self.render_targets.target_2 = Some(render_target(win_w, win_h));

            self.render_targets
                .target_1
                .as_ref()
                .unwrap()
                .texture
                .set_filter(FilterMode::Nearest);
            self.render_targets
                .target_2
                .as_ref()
                .unwrap()
                .texture
                .set_filter(FilterMode::Nearest);
        }
        
        if update_3 {
            self.render_targets.target_3 = Some(render_target(screen_w, screen_h));
            self.render_targets
                .target_3
                .as_ref()
                .unwrap()
                .texture
                .set_filter(FilterMode::Nearest);
        }
    }
}

/////////////////////////////////////
// SCOPES
/////////////////////////////////////

impl Window {
    pub fn scope(&mut self, mut f: impl FnMut(&mut Window)) -> &mut Window {
        f(self);
        self
    }

    pub async fn scope_async(&mut self, mut f: impl AsyncFnMut(&mut Window)) -> &mut Window {
        f(self).await;
        self
    }

    pub fn scope_if(
        &mut self,
        condition: impl Into<bool>,
        mut f: impl FnMut(&mut Window),
    ) -> &mut Window {
        if condition.into() {
            f(self)
        }
        self
    }

    pub async fn scope_async_if(
        &mut self,
        condition: impl Into<bool>,
        mut f: impl AsyncFnMut(&mut Window),
    ) -> &mut Window {
        if condition.into() {
            f(self).await;
        }
        self
    }
}

/////////////////////////////////////
// SAME LINE
/////////////////////////////////////

impl Window {
    // Helper to get or insert a WidgetHolder
    fn get_or_insert_holder(
        &mut self,
        id: impl Into<WidgetId>,
        same_line: bool,
    ) -> &mut WidgetHolder {
        let id_string = id.into().to_string();
        self.holder_ids.insert(id_string.clone()); // Ensure order
        self.widget_holders
            .entry(id_string)
            .or_insert_with(|| WidgetHolder::new(same_line))
    }

    pub fn same_line(
        &mut self,
        id: impl Into<WidgetId>,
        mut f: impl FnMut(&mut Window),
    ) -> &mut Window {
        let same_line_holder = self.get_or_insert_holder(id.into(), true);
        same_line_holder.same_line = true; // Ensure it's marked as same_line

        f(self);

        let next_holder_id = self.generate_widget_id("next_line");
        self.get_or_insert_holder(next_holder_id, false);
        self
    }

    pub async fn same_line_async(
        &mut self,
        id: impl Into<WidgetId>,
        mut f: impl AsyncFnMut(&mut Window),
    ) -> &mut Window {
        let same_line_holder = self.get_or_insert_holder(id.into(), true);
        same_line_holder.same_line = true;

        f(self).await;

        let next_holder_id = self.generate_widget_id("next_line_async");
        self.get_or_insert_holder(next_holder_id, false);
        self
    }

    pub fn same_line_if(
        &mut self,
        id: impl Into<WidgetId>,
        condition: impl Into<bool>,
        mut f: impl FnMut(&mut Window),
    ) -> &mut Window {
        let same_line_holder = self.get_or_insert_holder(id.into(), true);
        same_line_holder.same_line = true;

        if condition.into() {
            f(self)
        }

        let next_holder_id = self.generate_widget_id("next_line_if");
        self.get_or_insert_holder(next_holder_id, false);
        self
    }

    pub async fn same_line_async_if(
        &mut self,
        id: impl Into<WidgetId>,
        condition: impl Into<bool>,
        mut f: impl AsyncFnMut(&mut Window),
    ) -> &mut Window {
        let same_line_holder = self.get_or_insert_holder(id.into(), true);
        same_line_holder.same_line = true;

        if condition.into() {
            f(self).await;
        }

        let next_holder_id = self.generate_widget_id("next_line_async_if");
        self.get_or_insert_holder(next_holder_id, false);
        self
    }
}

/////////////////////////////////////
// WIDGET
/////////////////////////////////////

impl Window {
    pub fn begin_widgets(&mut self) {
        // Clear all holder_ids except the main one
        self.holder_ids.clear();
        self.holder_ids.insert(String::from("__Main__"));

        // Reset all existing widget holders
        for (_, holder) in self.widget_holders.iter_mut() {
            holder.reset();
        }
    }

    pub fn end_widgets(&mut self) {
        // Retain only the widget holders that were used in this frame
        self.widget_holders
            .retain(|k, _| self.holder_ids.contains(k));

        // Call retain() on the remaining holders
        for (_, holder) in self.widget_holders.iter_mut() {
            holder.retain();
        }
    }

    // GET LAST WIDGET HOLDER

    fn last_widget_holder(&mut self) -> &mut WidgetHolder {
        // If holder_ids is empty (shouldn't happen after begin_widgets, but defensive)
        if self.holder_ids.is_empty() {
            let main_id = String::from("__Main__");
            self.holder_ids.insert(main_id.clone());
            self.widget_holders
                .entry(main_id.clone())
                .or_insert_with(|| WidgetHolder::new(false));
        }
        let last_id = self.holder_ids.last().unwrap().clone(); // Clone to satisfy the borrow-checker
        self.widget_holders.get_mut(&last_id).unwrap()
    }

    // Helper to generate a unique WidgetId string for auto-IDs
    fn generate_widget_id(&self, prefix: &str) -> String {
        let mut hasher = DefaultHasher::new();
        prefix.hash(&mut hasher);
        self.holder_ids.len().hash(&mut hasher); // Use current number of holders for uniqueness
        format!("{}_{}", prefix, hasher.finish())
    }

    // WIDGET TYPES

    pub fn text(&mut self, label: impl ToString) -> &mut Text {
        let id = self.generate_widget_id("text");
        self.last_widget_holder().text(id.into(), label.to_string())
    }

    pub fn text_ex(
        &mut self,
        label: impl ToString,
        color: Color,
        font_size: u16,
        font: Option<Font>,
    ) -> &mut TextEx {
        let id = self.generate_widget_id("text_ex");
        self.last_widget_holder()
            .text_ex(id.into(), label.to_string(), color, font_size, font)
    }

    pub fn button(&mut self, id: impl Into<WidgetId>, label: impl ToString) -> &mut Button {
        self.last_widget_holder()
            .button(id.into(), label.to_string())
    }

    pub fn checkbox(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        default_value: bool,
    ) -> &mut Checkbox {
        self.last_widget_holder()
            .checkbox(id.into(), label.to_string(), default_value)
    }

    pub async fn image(
        &mut self,
        id: impl Into<WidgetId>,
        path: impl ToString,
        size: Option<Vec2>,
    ) -> &mut ImageWidget {
        self.last_widget_holder()
            .image(id.into(), path.to_string(), size)
            .await
    }

    pub fn slider(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        slider_info: SliderInfo,
    ) -> &mut Slider {
        self.last_widget_holder()
            .slider(id.into(), label.to_string(), slider_info)
    }

    pub fn progress_bar(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        progress_info: ProgressInfo,
    ) -> &mut ProgressBar {
        self.last_widget_holder()
            .progress_bar(id.into(), label.to_string(), progress_info)
    }

    pub fn dropdown(
        &mut self,
        id: impl Into<WidgetId>,
        items: Vec<impl ToString>,
        default_value: impl ToString,
    ) -> &mut Dropdown {
        let stringed_items: Vec<_> = items.iter().map(|x| x.to_string()).collect();
        let stringed_value = default_value.to_string();

        if !stringed_items.contains(&stringed_value) {
            println!("{stringed_value} not found in items [Dropdown].")
        }

        self.last_widget_holder()
            .dropdown(id.into(), stringed_items, stringed_value)
    }

    pub fn separator(&mut self) -> &mut Separator {
        self.last_widget_holder().separator(WidgetId::Auto)
    }

    pub fn tabs(
        &mut self,
        id: impl Into<WidgetId>,
        tabs: Vec<impl ToString>,
        default_tab: usize,
    ) -> &mut TabHolder {
        self.last_widget_holder().tabs(
            id.into(),
            tabs.iter().map(|x| x.to_string()).collect(),
            default_tab,
        )
    }

    pub fn textbox(
        &mut self,
        id: impl Into<WidgetId>,
        default_text: impl ToString,
    ) -> &mut TextBox {
        self.last_widget_holder()
            .textbox(id.into(), default_text.to_string())
    }

    pub fn labeled_textbox(
        &mut self,
        id: impl Into<WidgetId>,
        label: impl ToString,
        default_text: impl ToString,
    ) -> &mut TextBox {
        self.last_widget_holder().labeled_textbox(
            id.into(),
            label.to_string(),
            default_text.to_string(),
        )
    }
}
