use macroquad::{miniquad::CursorIcon::Help, prelude::*};
mod window;
use geometry::Rect;
use ui::Button;
use window::Window;

use crate::layout::{MenuAction, MenuItem, VerticalLayout};

mod geometry;
mod layout;
mod ui;
mod utils;

enum ContextMenuKind {
    Desktop,
    Window,
}

struct ContextMenu {
    rect: Rect,
    kind: ContextMenuKind,
    layout: VerticalLayout,
    is_open: bool,
    animation_progress: f32,
}

impl ContextMenu {
    pub fn new(x: f32, y: f32, width: f32, height: f32, kind: ContextMenuKind) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            kind,
            layout: VerticalLayout::new(Rect::new(x, y, width, height), 5.0),
            is_open: false,
            animation_progress: 0.0,
        }
    }
    pub fn draw(&mut self) {
        let animated_width = self.rect.width * self.animation_progress;
        let animated_height = self.rect.height * self.animation_progress;

        match self.kind {
            ContextMenuKind::Desktop => {
                draw_rectangle(
                    self.rect.x,
                    self.rect.y,
                    animated_width,
                    animated_height,
                    GREEN,
                );
            }
            ContextMenuKind::Window => {
                draw_rectangle(
                    self.rect.x,
                    self.rect.y,
                    animated_width,
                    animated_height,
                    RED,
                );
            }
        }

        self.layout.draw();
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.is_open = true;

        self.layout.rect.x = x;
        self.layout.rect.y = y;

        self.layout.items.clear();

        self.layout.add_item(MenuItem::new(
            Button::new(
                0.0,
                0.0,
                self.rect.width,
                30.0,
                "Settings".to_string(),
                PINK,
                None,
            ),
            MenuAction::Settings,
        ));

        self.layout.add_item(MenuItem::new(
            Button::new(
                0.0,
                0.0,
                self.rect.width,
                30.0,
                "New window".to_string(),
                ORANGE,
                None,
            ),
            MenuAction::NewWindow,
        ));

        self.animation_progress = 0.0;
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.rect.contains(x, y)
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn clicked_action(&self) -> Option<MenuAction> {
        self.layout.clicked_action()
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    set_pc_assets_folder("assets");
    // let sound = audio::load_sound("click.wav").await.unwrap();
    // let cat_texture = load_texture("cat.png")
    //     .await
    //     .expect("Error loading texture.");

    let mut windows: Vec<Window> = vec![];
    let mut context_menu = ContextMenu::new(0.0, 0.0, 100.0, 100.0, ContextMenuKind::Desktop);

    loop {
        let (mx, my) = mouse_position();
        let window_title = format!("Window {}", windows.len());

        if is_key_pressed(KeyCode::N) {
            windows.push(Window::new(
                100.0,
                100.0,
                400.0,
                400.0,
                window_title.clone(), // cat_texture.clone(),
                None,
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
            context_menu.animation_progress += 0.05;

            if context_menu.animation_progress > 1.0 {
                context_menu.animation_progress = 1.0;
            }
        }

        if context_menu.is_open {
            context_menu.draw();
        }

        if let Some(action) = context_menu.clicked_action() {
            match action {
                MenuAction::NewWindow => {
                    println!("NEW WINDOW");
                    windows.push(Window::new(
                        100.0,
                        100.0,
                        400.0,
                        400.0,
                        window_title.clone(), // cat_texture.clone(),
                        None,
                    ));
                    context_menu.close();
                }

                MenuAction::Settings => {
                    println!("SETTINGS");
                }

                MenuAction::CloseWindow => {
                    println!("CLOSE WINDOW");
                }
            }
        }

        next_frame().await
    }
}
