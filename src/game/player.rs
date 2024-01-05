use macroquad::prelude::*;

use super::{controls::Controls, level::Level};

pub struct Player {
    pub pos: Vec2,
    pub dim: Vec2,
    pub light_radius: usize,
}

impl Player {
    pub fn update(mut self, controls: &Controls, lvl: &Level) -> Self {
        let diag = (controls.is_right || controls.is_left) && (controls.is_up || controls.is_down);

        let norm_speed = 5.;
        let diag_speed = f32::sqrt((norm_speed * norm_speed) / 2.);

        let mut speed = vec2(0., 0.);
        if controls.is_right {
            speed.x = if diag { diag_speed } else { norm_speed };
        } else if controls.is_left {
            speed.x = if diag { -diag_speed } else { -norm_speed };
        }

        self.pos += speed;

        // keep player on non-solid blocks
        if lvl.is_solid_at(self.pos) {
            // put player back where they were
            self.pos -= speed;
        }

        speed.x = 0.;
        if controls.is_up {
            speed.y = if diag { -diag_speed } else { -norm_speed };
        } else if controls.is_down {
            speed.y = if diag { diag_speed } else { norm_speed };
        }

        self.pos += speed;

        // keep player on non-solid blocks
        if lvl.is_solid_at(self.pos) {
            // put player back where they were
            self.pos -= speed;
        }

        // self.pos.x = clamp(self.pos.x, self.dim.x / 2., lvl.dim.x - self.dim.x / 2.);
        // self.pos.y = clamp(self.pos.y, self.dim.y / 2., lvl.dim.y - self.dim.y / 2.);
        self
    }
}
