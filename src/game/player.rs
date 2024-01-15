use macroquad::prelude::*;

use super::gfx::StaticSprite;

pub struct Player<'a> {
    pub pos: Vec2,
    pub dim: Vec2,
    pub light_radius: usize,
    pub sprite: StaticSprite<'a>,
}

impl<'a> super::Game<'a> {
    pub(super) fn update_player(&mut self) {
        let player = &mut self.player;

        let controls = &self.controls;
        let lvl = &self.lvl;

        let is_diag =
            (controls.is_right || controls.is_left) && (controls.is_up || controls.is_down);

        let norm_speed = 5.;
        let diag_speed = f32::sqrt((norm_speed * norm_speed) / 2.);
        let effective_speed = if is_diag { diag_speed } else { norm_speed };

        let mut speed = vec2(0., 0.);
        if controls.is_right {
            speed.x = effective_speed;
        } else if controls.is_left {
            speed.x = -effective_speed;
        }

        player.pos += speed;

        // keep player on non-solid blocks
        if lvl.is_solid_at(player.pos) {
            // put player back where they were
            player.pos -= speed;
        }

        speed.x = 0.;
        if controls.is_up {
            speed.y = -effective_speed;
        } else if controls.is_down {
            speed.y = effective_speed;
        }

        player.pos += speed;

        // keep player on non-solid blocks
        if lvl.is_solid_at(player.pos) {
            // put player back where they were
            player.pos -= speed;
        }

        // self.pos.x = clamp(self.pos.x, self.dim.x / 2., lvl.dim.x - self.dim.x / 2.);
        // self.pos.y = clamp(self.pos.y, self.dim.y / 2., lvl.dim.y - self.dim.y / 2.);
    }
}
