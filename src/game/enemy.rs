use macroquad::prelude::*;

use super::{
    entity::{collides, is_out_of_lvl_bounds, Entity},
    gfx::{AnimatedSprite, Frame},
    player::Ability,
    resources::Resources,
};

pub enum EnemyStrategy {
    FollowPlayer,
    HorizontalPatrol,
    VerticalPatrol,
    NoMovement,
    Projectile,
}

pub struct Enemy<'a> {
    pub name: String,
    pub pos: Vec2,
    pub dim: Vec2,
    pub sprite: AnimatedSprite<'a>,
    pub strategy: EnemyStrategy,
    pub hp: usize,
    pub hp_max: usize,
    pub dir: Vec2,
    pub speed: f32,
    pub speed_solid: f32,
    pub fog_of_war: bool,
    pub damage: usize,
    pub melee: Ability,
    pub aggro_radius: f32,
    pub aggro_cooldown: Option<f64>,
    pub aggro_duration: f64,
}

impl<'a> Entity for Enemy<'a> {
    fn dim(&self) -> Vec2 {
        self.dim
    }

    fn pos(&self) -> Vec2 {
        self.pos
    }
}

fn collides_any(i: usize, enemies: &Vec<Enemy>) -> Option<usize> {
    for j in 0..enemies.len() {
        if i == j {
            continue;
        }
        if matches!(enemies[j].strategy, EnemyStrategy::Projectile) {
            continue;
        }
        if collides(&enemies[i], &enemies[j]) {
            return Some(j);
        }
    }
    None
}

fn count_non_projectiles(enemies: &Vec<Enemy>) -> usize {
    let mut count = 0;
    (0..enemies.len()).for_each(|i| {
        if !matches!(enemies[i].strategy, EnemyStrategy::Projectile) {
            count += 1;
        }
    });
    count
}

pub fn count_nexus(enemies: &Vec<Enemy>) -> usize {
    let mut count = 0;
    (0..enemies.len()).for_each(|i| {
        if enemies[i].name == "Nexus" {
            count += 1;
        }
    });
    count
}

fn is_dead(enemy: &Enemy) -> bool {
    enemy.hp == 0
}

impl<'a> super::Game<'a> {
    pub(super) fn update_enemies(&mut self) {
        let enemies = &mut self.enemies;
        let player = &mut self.player;
        let lvl = &self.lvl;
        let mut to_remove = vec![];
        let time = get_time();

        let enemy_count = count_non_projectiles(enemies);

        (0..enemies.len()).for_each(|i| {
            let mut damage_player = false;
            let ignore_solid_checks =
                lvl.is_solid_at(enemies[i].pos) || enemies[i].speed_solid > 0.;

            let speed = match lvl.is_solid_at(enemies[i].pos) {
                true => enemies[i].speed_solid,
                false => enemies[i].speed,
            };

            match enemies[i].strategy {
                EnemyStrategy::VerticalPatrol => {
                    enemies[i].dir.x = 0.;
                    if enemies[i].dir.y == 0. {
                        enemies[i].dir.y = speed;
                    }
                    enemies[i].pos.y += enemies[i].dir.y;

                    if collides(player, &enemies[i]) {
                        damage_player = true;
                    }

                    if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                        || collides_any(i, enemies).is_some()
                        || collides(player, &enemies[i])
                    {
                        // put back where they were, then reverse direction
                        enemies[i].pos.y -= enemies[i].dir.y;
                        enemies[i].dir.y *= -1.;
                    }
                }
                EnemyStrategy::HorizontalPatrol => {
                    enemies[i].dir.y = 0.;
                    if enemies[i].dir.x == 0. {
                        enemies[i].dir.x = speed;
                    }
                    enemies[i].pos.x += enemies[i].dir.x;

                    if collides(player, &enemies[i]) {
                        damage_player = true;
                    }

                    if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                        || collides_any(i, enemies).is_some()
                        || collides(player, &enemies[i])
                    {
                        // put back where they were, then reverse direction
                        enemies[i].pos.x -= enemies[i].dir.x;
                        enemies[i].dir.x *= -1.;
                    }
                }
                EnemyStrategy::FollowPlayer => {
                    if enemies[i].pos.distance(player.pos) < enemies[i].aggro_radius {
                        enemies[i].aggro_cooldown = Some(time + enemies[i].aggro_duration)
                    } else if enemies[i].aggro_cooldown.is_some_and(|cd| cd <= time) {
                        enemies[i].aggro_cooldown = None
                    }
                    if enemies[i].aggro_cooldown.is_some() {
                        let dir = (player.pos - enemies[i].pos).normalize();
                        enemies[i].pos.x += dir.x * speed;

                        if collides(player, &enemies[i]) {
                            damage_player = true;
                        }

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies).is_some()
                            || collides(player, &enemies[i])
                        {
                            // put player back where they were
                            enemies[i].pos.x -= dir.x * speed;
                        }

                        enemies[i].pos.y += dir.y * speed;

                        if collides(player, &enemies[i]) {
                            damage_player = true;
                        }

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies).is_some()
                            || collides(player, &enemies[i])
                        {
                            // put player back where they were
                            enemies[i].pos.y -= dir.y * speed;
                        }
                    }
                }
                EnemyStrategy::Projectile => {
                    enemies[i].pos.y += enemies[i].dir.y * speed;
                    enemies[i].pos.x += enemies[i].dir.x * speed;

                    let collided = collides_any(i, enemies);
                    if collided.is_some() {
                        let enemy_idx = collided.unwrap();
                        let mut damage = enemies[i].damage;
                        // make nexus take 10* damage if its the last enemy
                        if enemies[enemy_idx].name == "Nexus" && enemy_count == 1 {
                            damage *= 10;
                        }
                        enemies[enemy_idx].hp -= std::cmp::min(enemies[enemy_idx].hp, damage);
                        self.stats.damage_dealt += damage;
                        if enemies[enemy_idx].hp == 0 {
                            self.stats.enemies_killed += 1;
                        }
                        enemies[i].hp = 0;
                    }
                }
                EnemyStrategy::NoMovement => {
                    // nothing happens
                }
            }

            if damage_player && !enemies[i].melee.is_on_cooldown(time) {
                player.hp -= std::cmp::min(player.hp, enemies[i].damage);
                enemies[i].melee.last_use = Some(time);
                self.stats.damage_received += enemies[i].damage;
            }
            enemies[i].sprite.update();
        });

        (0..enemies.len()).for_each(|i| {
            if is_dead(&enemies[i]) || is_out_of_lvl_bounds(&enemies[i], &self.lvl) {
                to_remove.push(i)
            }
        });

        (0..to_remove.len()).rev().for_each(|i| {
            enemies.swap_remove(to_remove[i]);
        });
    }
}

pub fn create_enemy(name: String, pos: Vec2, res: &Resources) -> Enemy {
    let def = res.enemy_definitions.get(&name).unwrap();
    let tex = res.textures.get(&def.sprite.texture).unwrap();

    let mut frames = vec![];
    for frame_def in &def.sprite.frames {
        frames.push(Frame {
            texture: tex,
            dest_size: vec2(def.dim.x, def.dim.y),
            source_rect: Rect {
                x: frame_def.x,
                y: frame_def.y,
                w: def.dim.x,
                h: def.dim.y,
            },
        })
    }

    let strategy = match def.strategy.as_str() {
        "followPlayer" => EnemyStrategy::FollowPlayer,
        "verticalPatrol" => EnemyStrategy::VerticalPatrol,
        "horizontalPatrol" => EnemyStrategy::HorizontalPatrol,
        "noMovement" => EnemyStrategy::NoMovement,
        "projectile" => EnemyStrategy::Projectile,
        _ => panic!("invalid enemy strategy"),
    };

    Enemy {
        name: def.name.to_string(),
        speed: def.speed,
        speed_solid: def.speed_solid,
        strategy,
        pos,
        hp: def.hp_max,
        hp_max: def.hp_max,
        damage: def.damage,
        melee: Ability {
            name: "Attack".to_string(),
            cooldown: 0.5,
            last_use: None,
        },
        dim: vec2(def.dim.x, def.dim.y),
        sprite: AnimatedSprite {
            frames,
            fps: def.sprite.fps,
            frame_index: 0,
            time: 0.,
        },
        dir: vec2(0., 0.),
        fog_of_war: def.fog_of_war,
        aggro_radius: 160.,
        aggro_cooldown: None,
        aggro_duration: 5.0,
    }
}
