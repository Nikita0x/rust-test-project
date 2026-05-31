use macroquad::prelude::*;
mod window;
use window::Window;

mod geometry;
mod ui;

#[macroquad::main("MyGame")]
async fn main() {
    let mut window = Window::new(
        100.0,
        100.0,
        400.0,
        400.0,
        String::from("Hello from window"),
    );

    loop {
        window.update();
        window.draw();

        next_frame().await
    }
}
