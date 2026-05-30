use macroquad::{prelude::*, ui::widgets::Window};

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        let (x, y) = mouse_position();
        clear_background(GRAY);
        draw_grid(20, 1.0, BLACK, GRAY);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        draw_text("Lalala", 30.0, 30.0, 30.0, RED);
        draw_text(format!("X: {}, Y: {}", x, y), 90.0, 90.0, 30.0, RED);
        draw_text(screen_width().to_string(), 60.0, 60.0, 30.0, BLUE);

        struct Window {
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            title: String,
        }

        let mut window = Window {
            x: 100.0,
            y: 100.0,
            width: 400.0,
            height: 400.0,
            title: String::from("Hello from window"),
        };
        impl Window {
            fn draw(&mut self) {
                draw_rectangle(self.x, self.y, self.width, self.height, PURPLE);
                draw_line(
                    self.x,
                    self.y + 10.0,
                    self.x + self.width,
                    self.y + 10.0,
                    40.0,
                    BLUE,
                );
                draw_text(
                    self.title.clone(),
                    self.x + 20.0,
                    self.y + 20.0,
                    20.0,
                    BLACK,
                );

                let (x, y) = mouse_position();
                let mut is_dragging = false;

                if (is_mouse_button_down(MouseButton::Left)) {
                    is_dragging = true;
                } else {
                    is_dragging = false;
                }

                if (is_dragging) {
                    self.x = x;
                    self.y = y;
                }
            }
        }

        window.draw();

        next_frame().await
    }
}
