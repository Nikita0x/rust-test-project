use macroquad::prelude::*;

use crate::geometry::Rect;
use crate::ui::Button;

pub struct Window {
    pub x: f32,
    pub y: f32,
    pub width: f32,

    pub height: f32,

    pub title: String,

    pub is_dragging: bool,
    pub drag_offset_x: f32,
    pub drag_offset_y: f32,

    close_button: Button,
    expand_button: Button,
    minimize_button: Button,

    pub is_closed: bool,
}

impl Window {
    pub fn new(x: f32, y: f32, width: f32, height: f32, title: String) -> Self {
        let button_width = 40.0;
        let button_height = 40.0;

        Self {
            x,
            y,
            width,
            height,
            title,

            is_dragging: false,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,

            close_button: Button::new(
                x + width - button_width,
                y,
                button_width,
                button_height,
                "X".to_string(),
                RED,
            ),

            expand_button: Button::new(
                x + width - button_width * 2.0,
                y,
                button_width,
                button_height,
                "⛶".to_string(),
                YELLOW,
            ),

            minimize_button: Button::new(
                x + width - button_width * 3.0,
                y,
                button_width,
                button_height,
                "_".to_string(),
                ORANGE,
            ),

            is_closed: false,
        }
    }

    pub fn draw(&mut self) {
        if self.is_closed {
            return;
        }

        draw_rectangle(self.x, self.y, self.width, self.height, PURPLE);
        draw_rectangle(self.x, self.y, self.width, 40.0, BLUE);

        self.close_button.draw();
        self.expand_button.draw();
        self.minimize_button.draw();

        draw_text(&self.title, self.x + 20.0, self.y + 20.0, 20.0, BLACK);
    }

    pub fn update(&mut self) {
        self.close_button.update_state();
        self.expand_button.update_state();
        self.minimize_button.update_state();

        if self.close_button.is_clicked() {
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

            self.close_button
                .set_position(self.x + self.width - 40.0, self.y);

            self.expand_button
                .set_position(self.x + self.width - 80.0, self.y);

            self.minimize_button
                .set_position(self.x + self.width - 120.0, self.y);
        }
    }

    fn is_mouse_over_titlebar(&self) -> bool {
        let titlebar = Rect::new(self.x, self.y, self.width, 40.0);

        let (mouse_x, mouse_y) = mouse_position();

        titlebar.contains(mouse_x, mouse_y)
    }
}
