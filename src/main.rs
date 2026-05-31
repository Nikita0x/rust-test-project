use macroquad::{audio, prelude::*};
mod window;
use window::Window;

mod geometry;
mod ui;

#[macroquad::main("MyGame")]
async fn main() {
    set_pc_assets_folder("assets");
    let sound = audio::load_sound("click.wav").await.unwrap();

    let mut window = Window::new(
        100.0,
        100.0,
        400.0,
        400.0,
        String::from("Hello from window"),
        sound,
    );

    loop {
        window.update();
        window.draw();

        next_frame().await
    }
}
