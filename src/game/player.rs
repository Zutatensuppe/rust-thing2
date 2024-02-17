use macroquad::prelude::*;

use super::{
    enemy::{Enemy, EnemyStrategy},
    entity::{collides, Entity},
    gfx::StaticSprite,
    inventory::Inventory,
};

pub struct Ability {
    pub name: String,
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

    pub fn cooldown_left(&self, time: f64) -> f64 {
        if self.last_use.is_some() {
            let cooldown_end = self.last_use.unwrap() + self.cooldown;
            return cooldown_end - time;
        }
        0.0
    }
}

pub struct Player<'a> {
    pub pos: Vec2,
    pub target_pos: Option<Vec2>,
    pub dim: Vec2,
    pub light_radius: usize,
    pub sprite: StaticSprite<'a>,
    pub inventory: Inventory,
    pub speed: f32,
    pub hp: usize,
    pub hp_max: usize,

    pub auto: Ability,
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
        let player = &mut self.player;
        let enemies = &self.enemies;

        let controls = &self.controls;
        let lvl = &self.lvl;

        if controls.is_right_mouse_click || controls.is_right_mouse_down {
            player.target_pos = Some(self.controls.mouse_pos);
        }

        if player.target_pos.is_some() {
            let target_pos = player.target_pos.unwrap();
            let dist = target_pos.distance(player.pos);
            if dist < 5. {
                player.target_pos = None
            } else {
                let dir = (target_pos - player.pos).normalize();
                player.pos.x += dir.x * player.speed;

                // keep player on non-solid blocks
                if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
                    // put player back where they were
                    player.pos.x -= dir.x * player.speed;
                }

                player.pos.y += dir.y * player.speed;

                // keep player on non-solid blocks
                if lvl.is_solid_at(player.pos) || collides_any(player, enemies) {
                    // put player back where they were
                    player.pos.y -= dir.y * player.speed;
                }
            }
        }
    }
}
