use macroquad::{input::KeyCode::F1, miniquad::window, prelude::*};

mod geometry;
mod nodes;
use geometry::Rect;
use nodes::Node;

struct Context {
    show_debug_info: bool,
}

struct SelectionArea {
    enabled: bool,
    initial_position: (f32, f32),
    rect: Rect,
    nodes: Vec<Node>,
}

impl SelectionArea {
    pub fn new() -> Self {
        Self {
            initial_position: (0.0, 0.0),
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            enabled: false,
            nodes: vec![],
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.rect.width = width;
        self.rect.height = height;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let node = Node::new(Rect::new(50.0, 100.0, 100.0, 100.0));
    let mut context = Context {
        show_debug_info: true,
    };
    let mut selection_area = SelectionArea::new();

    loop {
        let (mx, my) = mouse_position();

        // keyboard bindings
        if is_key_pressed(KeyCode::Escape) {
            window::request_quit();
            println!("Quitting.");
        } else if is_key_pressed(KeyCode::F1) {
            println!("show debug info: {}", context.show_debug_info);
            context.show_debug_info = !context.show_debug_info;
        }

        if context.show_debug_info {
            draw_text(
                format!("Mouse position: x:{}, y:{}", mx, my),
                5.0,
                20.0,
                20.0,
                WHITE,
            );
            draw_text(
                format!(
                    "Selection: enabled:{}, nodes:{}",
                    selection_area.enabled,
                    selection_area.nodes.len()
                ),
                5.0,
                40.0,
                20.0,
                WHITE,
            );
        }

        if node.rect.is_clicked() {
            println!(
                "Info: width: {}, height: {}, x: {}, y: {}",
                node.rect.width, node.rect.height, node.rect.x, node.rect.y
            )
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            selection_area.enabled = true;
            selection_area.initial_position = (mx, my);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let x = selection_area.initial_position.0;
            let y = selection_area.initial_position.1;

            selection_area.set_position(x, y);
            selection_area.set_dimensions(mx - x, my - y);
            selection_area.rect.draw(Color::new(0.0, 0.0, 255.0, 0.20));
        } else {
            selection_area.enabled = false;
        }

        node.draw(&context, RED);

        next_frame().await
    }
}
