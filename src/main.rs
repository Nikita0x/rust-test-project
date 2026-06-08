use macroquad::{
    input::KeyCode::F1,
    miniquad::{CursorIcon::Help, window},
    prelude::*,
};

mod geometry;
mod nodes;
use geometry::Rect;
use nodes::Node;

struct Context {
    show_debug_info: bool,
    nodes: Vec<Node>,
}

struct SelectionArea {
    enabled: bool,
    initial_position: (f32, f32),
    rect: Rect,

    found_nodes: bool,

    // NEW: понимаем был ли реальный drag
    is_drag_selection: bool,

    selected_nodes: Vec<Node>,
}

impl SelectionArea {
    pub fn new() -> Self {
        Self {
            initial_position: (0.0, 0.0),
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            enabled: false,
            found_nodes: false,
            is_drag_selection: false,
            selected_nodes: vec![],
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

    pub fn contains_rect(&self, rect: &Rect) -> bool {
        self.rect.contains(rect.x, rect.y)
            && self.rect.contains(rect.x + rect.width, rect.y)
            && self.rect.contains(rect.x, rect.y + rect.height)
            && self
                .rect
                .contains(rect.x + rect.width, rect.y + rect.height)
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut context = Context {
        show_debug_info: true,
        nodes: vec![],
    };
    context
        .nodes
        .push(Node::new(Rect::new(50.0, 100.0, 100.0, 100.0)));
    context
        .nodes
        .push(Node::new(Rect::new(200.0, 200.0, 100.0, 100.0)));
    let mut selection_area = SelectionArea::new();

    loop {
        let (mx, my) = mouse_position();

        // keyboard bindings
        if is_key_pressed(KeyCode::Escape) {
            for node in &mut context.nodes {
                node.deselect();
            }
        }

        if is_key_pressed(KeyCode::F1) {
            println!("show debug info: {}", context.show_debug_info);
            context.show_debug_info = !context.show_debug_info;
        }

        // debug
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
                    selection_area.selected_nodes.len()
                ),
                5.0,
                40.0,
                20.0,
                WHITE,
            );
        }

        // selection
        if is_mouse_button_pressed(MouseButton::Left) {
            selection_area.enabled = true;
            selection_area.initial_position = (mx, my);

            selection_area.found_nodes = false;
            selection_area.is_drag_selection = false;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let start_x = selection_area.initial_position.0;
            let start_y = selection_area.initial_position.1;

            let x = start_x.min(mx);
            let y = start_y.min(my);

            let width = (mx - start_x).abs();
            let height = (my - start_y).abs();

            let drag_threshold = 5.0;

            if width > drag_threshold || height > drag_threshold {
                selection_area.is_drag_selection = true;
            }

            selection_area.set_position(x, y);
            selection_area.set_dimensions(width, height);

            if selection_area.is_drag_selection {
                selection_area.rect.draw(Color::new(0.0, 0.0, 255.0, 0.20));

                for node in &mut context.nodes {
                    if selection_area.contains_rect(&node.rect) {
                        node.select();
                        selection_area.found_nodes = true;
                    } else {
                        node.deselect();
                    }
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            selection_area.enabled = false;

            let clicked_on_node = context
                .nodes
                .iter()
                .any(|node| node.rect.contains_pointer());

            if !selection_area.is_drag_selection && !clicked_on_node {
                for node in &mut context.nodes {
                    node.deselect();
                }
            }
        }

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::A) {
            for node in &mut context.nodes {
                node.select();
            }
        }

        let mut clicked_index = None;

        for (i, node) in context.nodes.iter().enumerate() {
            if node.rect.is_clicked() {
                clicked_index = Some(i);
                break;
            }
        }

        if let Some(i) = clicked_index {
            let ctrl_pressed =
                is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

            if !ctrl_pressed {
                for node in &mut context.nodes {
                    node.deselect();
                }
            }

            context.nodes[i].select();
        }

        for node in &context.nodes {
            node.draw(&context, RED);
        }

        next_frame().await
    }
}
