use macroquad::prelude::*;
struct Window {
    x: f32,
    y: f32,
    width: f32,
    height: f32,

    title: String,

    button_width: f32,
    button_height: f32,

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

        let close_button_color = if self.is_mouse_over_close_button() {
            MAGENTA
        } else {
            RED
        };

        draw_rectangle(
            self.x + self.width - self.button_width,
            self.y,
            self.button_width,
            self.button_height,
            close_button_color,
        ); //close button

        let button_x = self.x + self.width - self.button_width;
        let button_y = self.y;

        let button_center_x = button_x + self.button_width / 2.0;
        let button_center_y = button_y + self.button_height / 2.0;

        draw_text(
            self.title.clone(),
            self.x + 20.0,
            self.y + 20.0,
            20.0,
            BLACK,
        );

        let close_button_text_dimensions = measure_text("X", None, 30, 1.0);

        draw_text(
            "X",
            button_center_x - close_button_text_dimensions.width / 2.0,
            button_center_y + close_button_text_dimensions.height / 2.0,
            30.0,
            BLACK,
        );
    }

    fn update(&mut self) {
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
            && mouse_y <= self.y + self.button_height
    }

    fn is_mouse_over_close_button(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        let button_x = self.x + self.width - self.button_width;
        let button_y = self.y;

        mouse_x >= button_x
            && mouse_x <= button_x + self.button_width
            && mouse_y >= button_y
            && mouse_y <= button_y + self.button_height
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
        button_height: 40.0,
        button_width: 40.0,
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

        window.update();
        window.draw();

        next_frame().await
    }
}
