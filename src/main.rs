use macroquad::prelude::*;
mod window;
use window::Window;

mod geometry;
mod ui;
mod utils;

#[macroquad::main("MyGame")]
async fn main() {
    set_pc_assets_folder("assets");
    // let sound = audio::load_sound("click.wav").await.unwrap();
    let cat_texture = load_texture("cat.png")
        .await
        .expect("Error loading texture.");

    let mut windows: Vec<Window> = vec![];

    loop {
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

        next_frame().await
    }
}
