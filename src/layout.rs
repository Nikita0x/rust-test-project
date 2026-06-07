use crate::geometry::Rect;
use crate::ui::Button;

pub struct VerticalLayout {
    pub rect: Rect,
    spacing: f32,
    pub items: Vec<MenuItem>, //TODO: заменить на более общий Widget
}

pub struct MenuItem {
    button: Button,
    action: MenuAction,
}

impl MenuItem {
    pub fn new(button: Button, action: MenuAction) -> Self {
        Self { button, action }
    }
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    NewWindow,
    Settings,
    CloseWindow,
}

impl VerticalLayout {
    pub fn new(rect: Rect, spacing: f32) -> Self {
        Self {
            rect,
            spacing,
            items: vec![],
        }
    }

    pub fn draw(&mut self) {
        let mut current_y = self.rect.y;

        for item in &mut self.items {
            item.button.set_position(self.rect.x, current_y);

            item.button.update_state();
            item.button.draw();

            current_y += item.button.rect.height + self.spacing;
        }
    }

    pub fn add_item(&mut self, menu_item: MenuItem) {
        self.items.push(menu_item);
    }

    pub fn clicked_action(&self) -> Option<MenuAction> {
        for item in &self.items {
            if item.button.is_clicked() {
                return Some(item.action);
            }
        }

        None
    }
}
