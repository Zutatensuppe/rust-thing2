use macroquad::prelude::*;

use super::{level::World, player::Player};

pub struct GameCamera {
    pub pos: Vec2,
    pub dim: Vec2,
}

impl GameCamera {
    pub fn update(mut self, world: &World, player: &Player) -> Self {
        // check if player is outside of the 'no movement' part of the camera
        let off_x = self.dim.x / 4.;
        let off_y = self.dim.y / 4.;

        let nomove_min_x = self.pos.x + off_x;
        let nomove_min_y = self.pos.y + off_y;
        let nomove_max_x = self.pos.x + self.dim.x - off_x;
        let nomove_max_y = self.pos.y + self.dim.y - off_y;
        if player.pos.x < nomove_min_x {
            self.pos.x += player.pos.x - nomove_min_x;
        } else if player.pos.x > nomove_max_x {
            self.pos.x += player.pos.x - nomove_max_x;
        }
        if player.pos.y < nomove_min_y {
            self.pos.y += player.pos.y - nomove_min_y;
        } else if player.pos.y > nomove_max_y {
            self.pos.y += player.pos.y - nomove_max_y;
        }

        self.pos = vec2(
            clamp(self.pos.x, 0., world.dim.x - self.dim.x),
            clamp(self.pos.y, 0., world.dim.y - self.dim.y),
        );
        self
    }
}
