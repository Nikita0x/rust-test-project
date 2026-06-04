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

    let mut window = Window::new(
        100.0,
        100.0,
        400.0,
        400.0,
        String::from("Hello from window"),
        cat_texture, // sound,
    );

    loop {
        window.update();
        window.draw();

        next_frame().await
    }
}
