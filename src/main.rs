use macroquad::prelude::*;
mod window;
use window::Window;

mod geometry;

#[macroquad::main("MyGame")]
async fn main() {
    let mut window = Window {
        x: 100.0,
        y: 100.0,
        width: 400.0,
        height: 400.0,
        title: String::from("Hello from window"),
        is_dragging: false,
        drag_offset_x: 0.0,
        drag_offset_y: 0.0,
        is_closed: false,
        button_height: 40.0,
        button_width: 40.0,
    };

    loop {
        window.update();
        window.draw();

        next_frame().await
    }
}
