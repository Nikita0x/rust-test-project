use macroquad::prelude::*;

use crate::geometry::Rect;
use crate::ui::Button;
use crate::utils::lerp;

#[derive(Debug)]
enum DockZone {
    None,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct Window {
    rect: Rect,
    target_rect: Rect,
    old_rect: Rect,

    pub title: String,

    is_dragging: bool,
    drag_offset_x: f32,
    drag_offset_y: f32,

    // sound: Sound,
    close_button: Button,
    expand_button: Button,
    minimize_button: Button,

    is_expanded: bool,
    is_docked: bool,
    is_closed: bool,
    preview_alpha: f32,
}

impl Window {
    pub fn new(x: f32, y: f32, width: f32, height: f32, title: String) -> Self {
        let button_width = 40.0;
        let button_height = 40.0;
        let is_expanded = false;
        let is_docked = false;

        Self {
            rect: Rect::new(x, y, width, height),
            old_rect: Rect::new(x, y, width, height),
            target_rect: Rect::new(x, y, width, height),

            is_expanded,
            is_docked,

            title,
            // sound,
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
            preview_alpha: 0.0,
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

    fn get_dock_zone(&self, mouse_x: f32, mouse_y: f32) -> DockZone {
        let margin = 10.0;

        let is_left = mouse_x <= margin;
        let is_right = mouse_x >= screen_width() - margin;
        let is_top = mouse_y <= margin;
        let is_bottom = mouse_y >= screen_height() - margin;

        if is_left && is_top {
            DockZone::TopLeft
        } else if is_right && is_top {
            DockZone::TopRight
        } else if is_left && is_bottom {
            DockZone::BottomLeft
        } else if is_right && is_bottom {
            DockZone::BottomRight
        } else if is_left {
            DockZone::Left
        } else if is_right {
            DockZone::Right
        } else if is_top {
            DockZone::Top
        } else if is_bottom {
            DockZone::Bottom
        } else {
            DockZone::None
        }
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
            self.is_expanded = !self.is_expanded;

            if self.is_expanded {
                self.old_rect = self.rect;

                self.target_rect.width = screen_width();
                self.target_rect.height = screen_height();
                self.set_window_position(0.0, 0.0);
            } else {
                self.target_rect.width = self.old_rect.width;
                self.target_rect.height = self.old_rect.height;
                self.set_window_position(self.old_rect.x, self.old_rect.y);
            }
        }
    }

    fn handle_close(&mut self) {
        if self.close_button.is_clicked() {
            // audio::play_sound_once(&self.sound);
            self.is_closed = true;
        }
    }
    fn handle_minimize(&mut self) {
        if self.minimize_button.is_clicked() {}
    }

    fn handle_drag(&mut self) {
        let hovering = self.is_mouse_over_titlebar();

        // 1. Start dragging logic
        if hovering
            && is_mouse_button_pressed(MouseButton::Left)
            && !self.expand_button.is_hovered()
            && !self.close_button.is_hovered()
            && !self.minimize_button.is_hovered()
        {
            let (mouse_x, mouse_y) = mouse_position();
            self.is_dragging = true;
            self.drag_offset_x = mouse_x - self.rect.x;
            self.drag_offset_y = mouse_y - self.rect.y;
        }

        // 2. While dragging logic
        if self.is_dragging {
            let (mouse_x, mouse_y) = mouse_position();
            let zone = self.get_dock_zone(mouse_x, mouse_y);

            // Check if we should stop dragging (mouse released)
            if is_mouse_button_released(MouseButton::Left) {
                self.is_dragging = false;
                self.preview_alpha = 0.0; // Reset preview on release

                // DOCKING: If released near the left edge, snap to the left half

                match zone {
                    DockZone::Left => {
                        self.target_rect.x = 0.0;
                        self.target_rect.y = 0.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height();
                        return;
                    }
                    DockZone::Right => {
                        self.target_rect.x = screen_width() / 2.0;
                        self.target_rect.y = 0.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height();
                        return;
                    }
                    DockZone::Top => {
                        self.target_rect.x = 0.0;
                        self.target_rect.y = 0.0;
                        self.target_rect.width = screen_width();
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    DockZone::Bottom => {
                        self.target_rect.x = 0.0;
                        self.target_rect.y = screen_height() / 2.0;
                        self.target_rect.width = screen_width();
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    DockZone::TopLeft => {
                        self.target_rect.x = 0.0;
                        self.target_rect.y = 0.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    DockZone::TopRight => {
                        self.target_rect.x = screen_width() / 2.0;
                        self.target_rect.y = 0.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    DockZone::BottomLeft => {
                        self.target_rect.x = 0.0;
                        self.target_rect.y = screen_height() / 2.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    DockZone::BottomRight => {
                        self.target_rect.x = screen_width() / 2.0;
                        self.target_rect.y = screen_height() / 2.0;
                        self.target_rect.width = screen_width() / 2.0;
                        self.target_rect.height = screen_height() / 2.0;
                        return;
                    }
                    _ => {}
                }

                // NORMAL RELEASE: Ensure target_rect matches where we dropped the window
                self.target_rect.x = self.rect.x;
                self.target_rect.y = self.rect.y;
                return;
            }

            // UPDATE POSITION: Follow the mouse
            self.rect.x = mouse_x - self.drag_offset_x;
            self.rect.y = mouse_y - self.drag_offset_y;

            // Keep target_rect in sync so it doesn't "snap back" to old position
            // if we were to stop dragging without a release (though release is handled above)
            self.target_rect.x = self.rect.x;
            self.target_rect.y = self.rect.y;

            // PREVIEW ANIMATION: Smoothly fade in/out the preview alpha
            // let target_alpha = if mouse_x <= 5.0 { 0.3 } else { 0.0 };
            let target_alpha = match zone {
                DockZone::None => 0.0,
                _ => 0.3,
            };
            self.preview_alpha = lerp(self.preview_alpha, target_alpha, 0.1);

            if self.preview_alpha > 0.001 {
                let color = Color::new(0.0, 0.5, 1.0, self.preview_alpha);

                // Рисуем превью в зависимости от зоны
                match zone {
                    DockZone::Left => {
                        draw_rectangle(0.0, 0.0, screen_width() / 2.0, screen_height(), color)
                    }
                    DockZone::Right => draw_rectangle(
                        screen_width() / 2.0,
                        0.0,
                        screen_width() / 2.0,
                        screen_height(),
                        color,
                    ),
                    DockZone::Top => {
                        draw_rectangle(0.0, 0.0, screen_width(), screen_height() / 2.0, color)
                    }
                    DockZone::Bottom => draw_rectangle(
                        0.0,
                        screen_height() / 2.0,
                        screen_width(),
                        screen_height() / 2.0,
                        color,
                    ),
                    DockZone::TopLeft => {
                        draw_rectangle(0.0, 0.0, screen_width() / 2.0, screen_height() / 2.0, color)
                    }
                    DockZone::TopRight => draw_rectangle(
                        screen_width() / 2.0,
                        0.0,
                        screen_width() / 2.0,
                        screen_height() / 2.0,
                        color,
                    ),
                    DockZone::BottomLeft => draw_rectangle(
                        0.0,
                        screen_height() / 2.0,
                        screen_width() / 2.0,
                        screen_height() / 2.0,
                        color,
                    ),
                    DockZone::BottomRight => draw_rectangle(
                        screen_width() / 2.0,
                        screen_height() / 2.0,
                        screen_width() / 2.0,
                        screen_height() / 2.0,
                        color,
                    ),
                    DockZone::None => {}
                }
            }
        } else {
            // Ensure preview fades out if we stop dragging for some reason
            self.preview_alpha = lerp(self.preview_alpha, 0.0, 0.1);
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
