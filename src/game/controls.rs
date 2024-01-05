use macroquad::prelude::*;

pub struct Controls {
    pub is_down: bool,
    pub is_up: bool,
    pub is_left: bool,
    pub is_right: bool,
}

impl Controls {
    pub fn update(mut self) -> Self {
        self.is_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.is_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.is_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.is_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        self
    }
}
