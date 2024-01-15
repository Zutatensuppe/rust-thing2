use macroquad::prelude::*;

use super::{gfx::AnimatedSprite, level::TILE_SIZE};

pub enum EnemyStrategy {
    FollowPlayer,
    HorizontalPatrol,
    VerticalPatrol,
}

pub struct Enemy<'a> {
    pub pos: Vec2,
    pub dim: Vec2,
    pub sprite: AnimatedSprite<'a>,
    pub strategy: EnemyStrategy,
    pub dir: Vec2,
    pub speed: f32,
    pub speed_solid: f32,
}

fn collides(enemy1: &Enemy, enemy2: &Enemy) -> bool {
    let rect1 = Rect {
        x: enemy1.pos.x - (enemy1.dim.x / 2.),
        y: enemy1.pos.y - (enemy1.dim.y / 2.),
        w: enemy1.dim.x,
        h: enemy1.dim.y,
    };
    let rect2 = Rect {
        x: enemy2.pos.x - (enemy2.dim.x / 2.),
        y: enemy2.pos.y - (enemy2.dim.y / 2.),
        w: enemy2.dim.x,
        h: enemy2.dim.y,
    };

    rect1.overlaps(&rect2)
}

fn collides_any(i: usize, enemies: &Vec<Enemy>) -> bool {
    for j in 0..enemies.len() {
        if i == j {
            continue;
        }
        if collides(&enemies[i], &enemies[j]) {
            return true;
        }
    }
    false
}

impl<'a> super::Game<'a> {
    pub(super) fn update_enemies(&mut self) {
        let enemies = &mut self.enemies;
        let player = &self.player;
        let lvl = &self.lvl;

        for i in 0..enemies.len() {
            match enemies[i].strategy {
                EnemyStrategy::VerticalPatrol => {
                    enemies[i].dir.x = 0.;

                    if enemies[i].dir.y == 0. {
                        enemies[i].dir.y = enemies[i].speed;
                    }

                    enemies[i].pos.y += enemies[i].dir.y;

                    // keep player on non-solid blocks
                    if lvl.is_solid_at(enemies[i].pos) || collides_any(i, enemies) {
                        // put player back where they were, then reverse direction
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

                    // keep player on non-solid blocks
                    if lvl.is_solid_at(enemies[i].pos) || collides_any(i, enemies) {
                        // put player back where they were, then reverse direction
                        enemies[i].pos.x -= enemies[i].dir.x;
                        enemies[i].dir.x *= -1.;
                    }
                }
                EnemyStrategy::FollowPlayer => {
                    let aggro_radius = 5. * TILE_SIZE;
                    if enemies[i].pos.distance(player.pos) < aggro_radius {
                        let ignore_solid_checks =
                            lvl.is_solid_at(enemies[i].pos) || enemies[i].speed_solid > 0.;
                        let norm_speed = if lvl.is_solid_at(enemies[i].pos) {
                            enemies[i].speed_solid
                        } else {
                            enemies[i].speed
                        };
                        let diag_speed = f32::sqrt((norm_speed * norm_speed) / 2.);

                        let is_diag =
                            enemies[i].pos.x != player.pos.x && enemies[i].pos.y != player.pos.y;
                        let effective_speed = if is_diag { diag_speed } else { norm_speed };

                        let mut speed = vec2(0., 0.);
                        if enemies[i].pos.x < player.pos.x {
                            speed.x = effective_speed;
                        } else if enemies[i].pos.x > player.pos.x {
                            speed.x = -effective_speed;
                        }

                        enemies[i].pos += speed;

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies)
                        {
                            // put player back where they were
                            enemies[i].pos -= speed;
                        }

                        speed.x = 0.;
                        if enemies[i].pos.y < player.pos.y {
                            speed.y = effective_speed;
                        } else if enemies[i].pos.y > player.pos.y {
                            speed.y = -effective_speed;
                        }

                        enemies[i].pos += speed;

                        // keep player on non-solid blocks
                        if (!ignore_solid_checks && lvl.is_solid_at(enemies[i].pos))
                            || collides_any(i, enemies)
                        {
                            // put player back where they were
                            enemies[i].pos -= speed;
                        }
                    }
                }
            }

            enemies[i].sprite.update();
        }
    }
}
