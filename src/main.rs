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
            windows.push(Window::new(
                100.0,
                100.0,
                400.0,
                400.0,
                format!("Window {}", windows.len()),
                cat_texture.clone(),
            ));
        }
        for window in windows.iter_mut() {
            window.update();
        }

        for window in windows.iter() {
            window.draw();
        }

        next_frame().await
    }
}
