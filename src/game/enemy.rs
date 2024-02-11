use macroquad::prelude::*;

use super::{
    entity::{collides, Entity},
    gfx::{AnimatedSprite, Frame},
    level::{Level, TILE_SIZE},
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
        if collides(&enemies[i], &enemies[j]) {
            return Some(j);
        }
    }
    None
}

fn is_dead(enemy: &Enemy) -> bool {
    enemy.hp == 0
}

fn is_out_of_lvl_bounds(enemy: &Enemy, lvl: &Level) -> bool {
    enemy.pos.x < 0.
        || enemy.pos.y < 0.
        || enemy.pos.x > (lvl.width as f32) * TILE_SIZE
        || enemy.pos.y > (lvl.height as f32) * TILE_SIZE
}

impl<'a> super::Game<'a> {
    pub(super) fn update_enemies(&mut self) {
        let enemies = &mut self.enemies;
        let player = &self.player;
        let lvl = &self.lvl;
        let mut to_remove = vec![];

        (0..enemies.len()).for_each(|i| {
            match enemies[i].strategy {
                EnemyStrategy::VerticalPatrol => {
                    enemies[i].dir.x = 0.;

                    if enemies[i].dir.y == 0. {
                        enemies[i].dir.y = enemies[i].speed;
                    }

                    enemies[i].pos.y += enemies[i].dir.y;

                    if lvl.is_solid_at(enemies[i].pos)
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
                        enemies[i].dir.x = enemies[i].speed;
                    }

                    enemies[i].pos.x += enemies[i].dir.x;

                    if lvl.is_solid_at(enemies[i].pos)
                        || collides_any(i, enemies).is_some()
                        || collides(player, &enemies[i])
                    {
                        // put back where they were, then reverse direction
                        enemies[i].pos.x -= enemies[i].dir.x;
                        enemies[i].dir.x *= -1.;
                    }
                }
                EnemyStrategy::FollowPlayer => {
                    let aggro_radius = 5. * 32.;
                    if enemies[i].pos.distance(player.pos) < aggro_radius {
                        let ignore_solid_checks =
                            lvl.is_solid_at(enemies[i].pos) || enemies[i].speed_solid > 0.;
                        let norm_speed = if lvl.is_solid_at(enemies[i].pos) {
                            enemies[i].speed_solid
                        } else {
                            enemies[i].speed
                        };

                        let dir = (player.pos - enemies[i].pos).normalize();
                        enemies[i].pos.x += dir.x * norm_speed;

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies).is_some()
                            || collides(player, &enemies[i])
                        {
                            // put player back where they were
                            enemies[i].pos.x -= dir.x * norm_speed;
                        }

                        enemies[i].pos.y += dir.y * norm_speed;

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies).is_some()
                            || collides(player, &enemies[i])
                        {
                            // put player back where they were
                            enemies[i].pos.y -= dir.y * norm_speed;
                        }
                    }
                }
                EnemyStrategy::Projectile => {
                    enemies[i].pos.y += enemies[i].dir.y * enemies[i].speed;
                    enemies[i].pos.x += enemies[i].dir.x * enemies[i].speed;

                    let collided = collides_any(i, enemies);
                    if collided.is_some() {
                        // TODO: dont hardcode the 10 dmg
                        let dmg = 10;
                        let enemy_idx = collided.unwrap();
                        enemies[enemy_idx].hp -= std::cmp::min(enemies[enemy_idx].hp, dmg);
                        enemies[i].hp = 0;
                    }
                }
                EnemyStrategy::NoMovement => {
                    // nothing happens
                }
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
        speed: def.speed,
        speed_solid: def.speed_solid,
        strategy,
        pos,
        hp: def.hp_max,
        hp_max: def.hp_max,
        dim: vec2(def.dim.x, def.dim.y),
        sprite: AnimatedSprite {
            frames,
            fps: def.sprite.fps,
            frame_index: 0,
            time: 0.,
        },
        dir: vec2(0., 0.),
        fog_of_war: def.fog_of_war,
    }
}
