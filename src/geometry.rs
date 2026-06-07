use macroquad::{
    color::Color,
    input::{is_mouse_button_pressed, mouse_position},
    shapes::draw_rectangle,
};

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }

    fn left(&self) -> f32 {
        self.x
    }
    fn right(&self) -> f32 {
        self.x + self.width
    }
    fn top(&self) -> f32 {
        self.y
    }
    fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.left() && x <= self.right() && y >= self.top() && y <= self.bottom()
    }

    pub fn contains_pointer(&self) -> bool {
        let (mx, my) = mouse_position();
        self.contains(mx, my)
    }

    pub fn is_hovered(&self) -> bool {
        self.contains_pointer()
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(macroquad::input::MouseButton::Left)
    }
}
