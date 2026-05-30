use macroquad::prelude::*;
struct Window {
    x: f32,
    y: f32,
    width: f32,
    height: f32,

    title: String,

    is_dragging: bool,
    drag_offset_x: f32,
    drag_offset_y: f32,

    is_closed: bool,
}

impl Window {
    fn draw(&mut self) {
        if self.is_closed {
            return;
        }

        draw_rectangle(self.x, self.y, self.width, self.height, PURPLE); //window
        draw_rectangle(self.x, self.y, self.width, 40.0, BLUE); //titlebar
        draw_rectangle(self.x + self.width - 40.0, self.y, 40.0, 40.0, RED); //close button
        draw_text(
            self.title.clone(),
            self.x + 20.0,
            self.y + 20.0,
            20.0,
            BLACK,
        );

        let hovering = self.is_mouse_over_titlebar();

        if hovering && is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            self.is_dragging = true;

            self.drag_offset_x = mouse_x - self.x;
            self.drag_offset_y = mouse_y - self.y;
        }

        if self.is_mouse_over_close_button() && is_mouse_button_pressed(MouseButton::Left) {
            self.is_closed = true;
            println!("Window closed.")
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let (mouse_x, mouse_y) = mouse_position();

            self.x = mouse_x - self.drag_offset_x;
            self.y = mouse_y - self.drag_offset_y;
        }
    }

    fn is_mouse_over_titlebar(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + 40.0
    }

    fn is_mouse_over_close_button(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        let button_x = self.x + self.width - 40.0;
        let button_y = self.y;
        let button_width = 40.0;
        let button_height = 40.0;

        mouse_x >= button_x
            && mouse_x <= button_x + button_width
            && mouse_y >= button_y
            && mouse_y <= button_y + button_height
    }
}

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
    };

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

        window.draw();

        next_frame().await
    }
}
