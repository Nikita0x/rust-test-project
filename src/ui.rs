use macroquad::prelude::*;

use crate::geometry::Rect;

pub struct Button {
    rect: Rect,
    text: String,
    color: Color,
    state: ButtonState,
}

pub enum ButtonState {
    Normal,
    Hovered,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: String, color: Color) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            text,
            color,
            state: ButtonState::Normal,
        }
    }

    pub fn contains(&self, mouse_x: f32, mouse_y: f32) -> bool {
        self.rect.contains(mouse_x, mouse_y)
    }

    pub fn draw(&self) {
        let color = match self.state {
            ButtonState::Normal => self.color,
            ButtonState::Hovered => MAGENTA,
        };

        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            color,
        );

        let text_dimensions = measure_text(&self.text, None, 30, 1.0);

        let text_x = self.rect.x + self.rect.width / 2.0 - text_dimensions.width / 2.0;

        let text_y = self.rect.y + self.rect.height / 2.0 + text_dimensions.height / 2.0;
        draw_text(&self.text, text_x, text_y, 30.0, BLACK);
    }

    pub fn is_hovered(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        self.contains(mouse_x, mouse_y)
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn update_state(&mut self) {
        if self.is_hovered() {
            self.state = ButtonState::Hovered;
        } else {
            self.state = ButtonState::Normal;
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}
