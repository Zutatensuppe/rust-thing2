use macroquad::prelude::*;

pub struct Controls {
    pub is_down: bool,
    pub is_up: bool,
    pub is_left: bool,
    pub is_right: bool,
    pub is_q: bool,

    pub mouse_pos: Vec2,
}

impl<'a> super::Game<'a> {
    pub(super) fn update_controls(&mut self) {
        let mouse_pos = mouse_position();
        let game_off = self.offset();
        self.controls.mouse_pos = vec2(mouse_pos.0 - game_off.x, mouse_pos.1 - game_off.y);
        self.controls.is_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.controls.is_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.controls.is_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.controls.is_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        self.controls.is_q = is_key_down(KeyCode::Q);
    }
}
