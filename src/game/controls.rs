use macroquad::prelude::*;

pub struct Controls {
    pub is_q: bool,

    pub mouse_pos: Vec2,
    pub is_mouse_left: bool,
    pub is_mouse_right: bool,
}

impl<'a> super::Game<'a> {
    pub(super) fn update_controls(&mut self) {
        let mouse_pos = mouse_position();
        let game_off = self.offset();
        self.controls.mouse_pos = vec2(mouse_pos.0 - game_off.x, mouse_pos.1 - game_off.y);
        self.controls.is_mouse_left = is_mouse_button_down(MouseButton::Left);
        self.controls.is_mouse_right = is_mouse_button_down(MouseButton::Right);
        self.controls.is_q = is_key_down(KeyCode::Q);
    }
}
