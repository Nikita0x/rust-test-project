use macroquad::{
    color::{BLACK, BLUE, Color, RED},
    shapes::draw_rectangle,
    text::draw_text,
};

use crate::{Context, geometry::Rect};

pub struct Node {
    pub rect: Rect,
    is_selected: bool,
}

impl Node {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            is_selected: false,
        }
    }

    pub fn draw(&self, context: &Context, color: Color) {
        self.rect.draw(color);

        if context.show_debug_info {
            draw_text(
                format!("Width: {}", self.rect.width),
                self.rect.x,
                self.rect.y + 20.0,
                20.0,
                BLACK,
            );
            draw_text(
                format!("Height: {}", self.rect.height),
                self.rect.x,
                self.rect.y + 40.0,
                20.0,
                BLACK,
            );
            draw_text(
                format!("x: {}", self.rect.x),
                self.rect.x,
                self.rect.y + 60.0,
                20.0,
                BLACK,
            );
            draw_text(
                format!("y: {}", self.rect.y),
                self.rect.x,
                self.rect.y + 80.0,
                20.0,
                BLACK,
            );
            draw_text(
                format!("sel: {}", self.is_selected),
                self.rect.x,
                self.rect.y + 100.0,
                20.0,
                BLACK,
            );
        }
    }
}
