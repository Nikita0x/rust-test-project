use macroquad::prelude::*;

use crate::geometry::Rect;

pub struct Window {
    pub x: f32,
    pub y: f32,
    pub width: f32,

    pub height: f32,

    pub title: String,
    pub button_width: f32,

    pub button_height: f32,
    pub is_dragging: bool,
    pub drag_offset_x: f32,
    pub drag_offset_y: f32,

    pub is_closed: bool,
}

impl Window {
    pub fn draw(&mut self) {
        if self.is_closed {
            return;
        }

        draw_rectangle(self.x, self.y, self.width, self.height, PURPLE); //window
        draw_rectangle(self.x, self.y, self.width, 40.0, BLUE); //titlebar

        let close_button = crate::ui::Button::new(
            self.x + self.width - self.button_width,
            self.y,
            self.button_width,
            self.button_height,
            "X".to_string(),
            RED,
        );

        close_button.draw();

        let expand_button = crate::ui::Button::new(
            self.x + self.width - self.button_width * 2.0,
            self.y,
            self.button_width,
            self.button_height,
            "⛶".to_string(),
            YELLOW,
        );

        expand_button.draw();

        let minimize_button = crate::ui::Button::new(
            self.x + self.width - self.button_width * 3.0,
            self.y,
            self.button_width,
            self.button_height,
            "_".to_string(),
            ORANGE,
        );

        minimize_button.draw();

        draw_text(&self.title, self.x + 20.0, self.y + 20.0, 20.0, BLACK);
    }

    pub fn update(&mut self) {
        let close_button = crate::ui::Button::new(
            self.x + self.width - self.button_width,
            self.y,
            self.button_width,
            self.button_height,
            "X".to_string(),
            RED,
        );

        if close_button.is_clicked() {
            self.is_closed = true;
        }

        let hovering = self.is_mouse_over_titlebar();

        if hovering && is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            self.is_dragging = true;

            self.drag_offset_x = mouse_x - self.x;
            self.drag_offset_y = mouse_y - self.y;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let (mouse_x, mouse_y) = mouse_position();

            self.x = mouse_x - self.drag_offset_x;
            self.y = mouse_y - self.drag_offset_y;
        }
    }

    fn is_mouse_over_titlebar(&self) -> bool {
        let titlebar = Rect::new(self.x, self.y, self.width, self.button_height);

        let (mouse_x, mouse_y) = mouse_position();

        titlebar.contains(mouse_x, mouse_y)
    }
}
