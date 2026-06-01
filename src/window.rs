use macroquad::audio::Sound;
use macroquad::{audio, prelude::*};

use crate::geometry::Rect;
use crate::ui::Button;
use crate::utils::lerp;

pub struct Window {
    rect: Rect,
    target_rect: Rect,
    old_rect: Rect,

    pub title: String,

    is_dragging: bool,
    drag_offset_x: f32,
    drag_offset_y: f32,

    sound: Sound,

    close_button: Button,
    expand_button: Button,
    minimize_button: Button,

    is_expanded: bool,
    is_closed: bool,
}

impl Window {
    pub fn new(x: f32, y: f32, width: f32, height: f32, title: String, sound: Sound) -> Self {
        let button_width = 40.0;
        let button_height = 40.0;
        let is_expanded = false;

        Self {
            rect: Rect::new(x, y, width, height),
            old_rect: Rect::new(x, y, width, height),
            target_rect: Rect::new(x, y, width, height),

            is_expanded,

            title,
            sound,

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

        let content_area = draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            PURPLE,
        );

        let titlebar = draw_rectangle(self.rect.x, self.rect.y, self.rect.width, 40.0, BLUE);

        self.close_button.draw();
        self.expand_button.draw();
        self.minimize_button.draw();

        let title = draw_text(
            &self.title,
            self.rect.x + 20.0,
            self.rect.y + 20.0,
            20.0,
            BLACK,
        );
    }

    pub fn update(&mut self) {
        self.close_button.update_state();
        self.expand_button.update_state();
        self.minimize_button.update_state();

        self.handle_minimize();
        self.handle_expand();
        self.handle_close();
        self.handle_drag();

        self.animate();
    }

    fn is_mouse_over_titlebar(&self) -> bool {
        let titlebar = Rect::new(self.rect.x, self.rect.y, self.rect.width, 40.0);

        let (mouse_x, mouse_y) = mouse_position();

        titlebar.contains(mouse_x, mouse_y)
    }

    fn set_window_position(&mut self, x: f32, y: f32) {
        self.target_rect.x = x;
        self.target_rect.y = y;
    }

    fn update_buttons_position(&mut self) {
        self.close_button
            .set_position(self.rect.x + self.rect.width - 40.0, self.rect.y);

        self.expand_button
            .set_position(self.rect.x + self.rect.width - 80.0, self.rect.y);

        self.minimize_button
            .set_position(self.rect.x + self.rect.width - 120.0, self.rect.y);
    }

    fn handle_expand(&mut self) {
        if self.expand_button.is_clicked() {
            println!("Expand button clicked.");
            self.is_expanded = !self.is_expanded;

            if self.is_expanded {
                println!("Expanding....");
                self.old_rect.x = self.rect.x;
                self.old_rect.y = self.rect.y;
                self.old_rect.width = self.rect.width;
                self.old_rect.height = self.rect.height;

                self.target_rect.width = screen_width();
                self.target_rect.height = screen_height();
                self.set_window_position(0.0, 0.0);
            } else {
                println!("Shrinking....");
                self.target_rect.width = self.old_rect.width;
                self.target_rect.height = self.old_rect.height;
                self.set_window_position(self.old_rect.x, self.old_rect.y);
            }
        }
    }

    fn handle_close(&mut self) {
        if self.close_button.is_clicked() {
            audio::play_sound_once(&self.sound);
            self.is_closed = true;
        }
    }
    fn handle_minimize(&mut self) {
        if self.minimize_button.is_clicked() {
            println!("minimize btn clicked");
        }
    }

    fn handle_drag(&mut self) {
        let hovering = self.is_mouse_over_titlebar();

        if hovering
            && is_mouse_button_pressed(MouseButton::Left)
            && !self.expand_button.is_hovered()
        {
            let (mouse_x, mouse_y) = mouse_position();

            self.is_dragging = true;

            self.drag_offset_x = mouse_x - self.rect.x;
            self.drag_offset_y = mouse_y - self.rect.y;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let (mouse_x, mouse_y) = mouse_position();

            self.rect.x = mouse_x - self.drag_offset_x;
            self.rect.y = mouse_y - self.drag_offset_y;

            self.target_rect.x = self.rect.x;
            self.target_rect.y = self.rect.y;
        }
    }

    fn animate(&mut self) {
        let t = 0.1;

        self.rect.width = lerp(self.rect.width, self.target_rect.width, t);

        self.rect.height = lerp(self.rect.height, self.target_rect.height, t);

        self.rect.x = lerp(self.rect.x, self.target_rect.x, t);

        self.rect.y = lerp(self.rect.y, self.target_rect.y, t);

        self.update_buttons_position();
    }
}
