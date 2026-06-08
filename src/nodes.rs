use macroquad::{
    color::{BLACK, BLUE, Color, RED},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};
use uuid::Uuid;

use crate::{Context, geometry::Rect};

#[derive(Copy, Clone)]
pub struct Node {
    id: Uuid,
    pub rect: Rect,
    pub is_selected: bool,
}

impl Node {
    pub fn new(rect: Rect) -> Self {
        Self {
            id: Uuid::new_v4(),
            rect,
            is_selected: false,
        }
    }

    pub fn draw(&self, context: &Context, color: Color) {
        self.rect.draw(color);

        //outline when selected
        if self.is_selected {
            let padding = 5.0;

            draw_rectangle_lines(
                self.rect.x - padding,
                self.rect.y - padding,
                self.rect.width + padding * 2.0,
                self.rect.height + padding * 2.0,
                2.0,
                BLUE,
            );
        }

        // if debug enabled
        if context.show_debug_info {
            // draw_text(
            //     format!("Width: {}", self.rect.width),
            //     self.rect.x,
            //     self.rect.y + 20.0,
            //     20.0,
            //     BLACK,
            // );
            // draw_text(
            //     format!("Height: {}", self.rect.height),
            //     self.rect.x,
            //     self.rect.y + 40.0,
            //     20.0,
            //     BLACK,
            // );
            draw_text(
                format!("id: {}", self.id),
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

    pub fn select(&mut self) {
        self.is_selected = true;
    }

    pub fn deselect(&mut self) {
        self.is_selected = false;
    }
}
