use macroquad::prelude::*;

pub struct Controls {
    pub is_down: bool,
    pub is_up: bool,
    pub is_left: bool,
    pub is_right: bool,
}

impl<'a> super::Game<'a> {
    pub(super) fn update_controls(&mut self) {
        self.controls.is_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.controls.is_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.controls.is_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.controls.is_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
    }
}
