use macroquad::prelude::*;

use super::{
    enemy::Enemy,
    entity::{collides, Entity},
    gfx::StaticSprite,
    inventory::Inventory,
};

pub struct Player<'a> {
    pub pos: Vec2,
    pub dim: Vec2,
    pub light_radius: usize,
    pub sprite: StaticSprite<'a>,
    pub inventory: Inventory,
}

impl<'a> Entity for Player<'a> {
    fn dim(&self) -> Vec2 {
        self.dim
    }

    fn pos(&self) -> Vec2 {
        self.pos
    }
}

fn collides_any(player: &Player, enemies: &Vec<Enemy>) -> bool {
    for enemy in enemies {
        if collides(player, enemy) {
            return true;
        }
    }
    false
}

impl<'a> super::Game<'a> {
    pub(super) fn update_player(&mut self) {
        let player = &mut self.player;
        let enemies = &self.enemies;

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
        if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
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
        if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
            // put player back where they were
            player.pos -= speed;
        }
    }
}
