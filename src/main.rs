use macroquad::{miniquad::CursorIcon::Help, prelude::*};
mod window;
use geometry::Rect;
use ui::Button;
use window::Window;

mod geometry;
mod ui;
mod utils;

enum ContextMenuKind {
    Desktop,
    Window,
}

struct ContextMenu {
    rect: Rect,
    kind: ContextMenuKind,
    is_open: bool,
}

impl ContextMenu {
    pub fn new(x: f32, y: f32, width: f32, height: f32, kind: ContextMenuKind) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            kind,
            is_open: false,
        }
    }
    pub fn draw(&self) {
        match self.kind {
            ContextMenuKind::Desktop => {
                draw_rectangle(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    GREEN,
                );
            }
            ContextMenuKind::Window => {
                draw_rectangle(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    RED,
                );
            }
        }
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.is_open = true;
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.rect.contains(x, y)
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    set_pc_assets_folder("assets");
    // let sound = audio::load_sound("click.wav").await.unwrap();
    let cat_texture = load_texture("cat.png")
        .await
        .expect("Error loading texture.");

    let mut windows: Vec<Window> = vec![];
    let mut context_menu = ContextMenu::new(0.0, 0.0, 100.0, 100.0, ContextMenuKind::Desktop);

    loop {
        let (mx, my) = mouse_position();

        if is_key_pressed(KeyCode::N) {
            let title = format!("Window {}", windows.len());
            windows.push(Window::new(
                100.0,
                100.0,
                400.0,
                400.0,
                title,
                cat_texture.clone(),
            ));
        }

        let mut clicked_index = None;
        for (i, window) in windows.iter().enumerate().rev() {
            if window.is_clicked() {
                clicked_index = Some(i);
                break;
            }
        }

        if let Some(index) = clicked_index {
            let window = windows.remove(index);
            windows.push(window);
        }

        let len = windows.len();
        for (i, window) in windows.iter_mut().enumerate() {
            let is_active = i == len - 1;
            window.update(is_active);
        }

        for (i, window) in windows.iter().enumerate() {
            let is_active = i == len - 1;
            window.draw(is_active);
        }

        if context_menu.is_open
            && !context_menu.contains(mx, my)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            context_menu.close();
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();

            context_menu.kind = if windows.iter().any(|window| window.is_hovered()) {
                ContextMenuKind::Window
            } else {
                ContextMenuKind::Desktop
            };

            context_menu.open_at(mx, my);
        }

        if context_menu.is_open {
            context_menu.draw();
        }

        next_frame().await
    }
}
