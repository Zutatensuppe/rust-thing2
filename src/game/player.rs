use macroquad::prelude::*;

use super::{
    enemy::{Enemy, EnemyStrategy},
    entity::{collides, Entity},
    gfx::StaticSprite,
    inventory::Inventory,
    util::calc_diag_speed,
};

pub struct Ability {
    pub cooldown: f64,
    pub last_use: Option<f64>,
}

impl Ability {
    pub fn is_on_cooldown(&self, time: f64) -> bool {
        if self.last_use.is_some() {
            let cooldown_end = self.last_use.unwrap() + self.cooldown;
            return cooldown_end > time;
        }
        false
    }
}

pub struct Player<'a> {
    pub pos: Vec2,
    pub dim: Vec2,
    pub dir: Vec2,
    pub light_radius: usize,
    pub sprite: StaticSprite<'a>,
    pub inventory: Inventory,

    pub q: Ability,
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
        if matches!(enemy.strategy, EnemyStrategy::Projectile) {
            continue;
        }
        if collides(player, enemy) {
            return true;
        }
    }
    false
}

impl<'a> super::Game<'a> {
    pub(super) fn update_player(&mut self) {
        // TODO: dont hardcode speed
        let norm_speed = 3.;

        let player = &mut self.player;
        let enemies = &self.enemies;

        let controls = &self.controls;
        let lvl = &self.lvl;

        if controls.is_mouse_right {
            player.dir = (self.controls.mouse_pos - player.pos).normalize();

            player.pos.x += player.dir.x * norm_speed;

            // keep player on non-solid blocks
            if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
                // put player back where they were
                player.pos.x -= player.dir.x * norm_speed;
            }

            player.pos.y += player.dir.y * norm_speed;

            // keep player on non-solid blocks
            if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
                // put player back where they were
                player.pos.y -= player.dir.y * norm_speed;
            }
        } else {
            // dont move
        }
    }
}
