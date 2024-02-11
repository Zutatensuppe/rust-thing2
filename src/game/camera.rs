use macroquad::prelude::*;

pub struct GameCamera {
    pub pos: Vec2,
    pub dim: Vec2,
}

impl GameCamera {}

impl<'a> super::Game<'a> {
    pub(super) fn update_camera(&mut self) {
        let cam = &mut self.camera;

        let player = &self.player;
        let world = &self.world;

        // check if player is outside of the 'no movement' part of the camera
        let off_x = cam.dim.x / 4.;
        let off_y = cam.dim.y / 4.;

        let nomove_min_x = cam.pos.x + off_x;
        let nomove_min_y = cam.pos.y + off_y;
        let nomove_max_x = cam.pos.x + cam.dim.x - off_x;
        let nomove_max_y = cam.pos.y + cam.dim.y - off_y;
        if player.pos.x < nomove_min_x {
            cam.pos.x += player.pos.x - nomove_min_x;
        } else if player.pos.x > nomove_max_x {
            cam.pos.x += player.pos.x - nomove_max_x;
        }
        if player.pos.y < nomove_min_y {
            cam.pos.y += player.pos.y - nomove_min_y;
        } else if player.pos.y > nomove_max_y {
            cam.pos.y += player.pos.y - nomove_max_y;
        }

        cam.pos = vec2(
            clamp(cam.pos.x, 0., world.dim.x - cam.dim.x),
            clamp(cam.pos.y, 0., world.dim.y - cam.dim.y),
        );
    }
}
