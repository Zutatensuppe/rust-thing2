mod game;

use game::camera::GameCamera;
use game::controls::Controls;
use game::enemy::{Enemy, EnemyStrategy};
use game::gfx::{AnimatedSprite, Frame, StaticSprite};
use game::level::{FogLevel, Level, World, TILE_SIZE};
use game::player::Player;
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "rust-thing2".to_owned(),
        fullscreen: false,
        // window_width: 320,
        // window_height: 240,
        window_resizable: false,
        ..Default::default()
    }
}

const BLOB_OFFSET: f32 = 0.;
const FOX_OFFSET: f32 = 32.;
const HYOTTOKO_OFFSET: f32 = 64.;
const SPIDER_OFFSET: f32 = 96.;

fn create_blob(x: f32, y: f32, spritepack: &Texture2D) -> Enemy {
    Enemy {
        speed: 2.,
        speed_solid: 0.,
        strategy: EnemyStrategy::VerticalPatrol,
        pos: vec2(x, y),
        dim: vec2(TILE_SIZE, TILE_SIZE),
        sprite: AnimatedSprite {
            frames: vec![
                Frame {
                    texture: spritepack,
                    source_rect: Rect {
                        x: BLOB_OFFSET,
                        y: 0.,
                        w: TILE_SIZE,
                        h: TILE_SIZE,
                    },
                    dest_size: Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    },
                },
                Frame {
                    texture: spritepack,
                    source_rect: Rect {
                        x: BLOB_OFFSET,
                        y: TILE_SIZE,
                        w: TILE_SIZE,
                        h: TILE_SIZE,
                    },
                    dest_size: Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    },
                },
            ],
            fps: 5,
            frame_index: 0,
            time: 0.,
        },
        dir: vec2(0., 0.),
    }
}

fn create_spider(x: f32, y: f32, spritepack: &Texture2D) -> Enemy {
    Enemy {
        strategy: EnemyStrategy::FollowPlayer,
        speed: 1.,
        speed_solid: 0.5,
        pos: vec2(x, y),
        dim: vec2(TILE_SIZE, TILE_SIZE),
        sprite: AnimatedSprite {
            frames: vec![
                Frame {
                    texture: spritepack,
                    source_rect: Rect {
                        x: SPIDER_OFFSET,
                        y: 0.,
                        w: TILE_SIZE,
                        h: TILE_SIZE,
                    },
                    dest_size: Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    },
                },
                Frame {
                    texture: spritepack,
                    source_rect: Rect {
                        x: SPIDER_OFFSET,
                        y: TILE_SIZE,
                        w: TILE_SIZE,
                        h: TILE_SIZE,
                    },
                    dest_size: Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    },
                },
            ],
            fps: 5,
            frame_index: 0,
            time: 0.,
        },
        dir: vec2(0., 0.),
    }
}

fn draw_frame(frame: &Frame, x: f32, y: f32) {
    draw_texture_ex(
        frame.texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(frame.source_rect),
            dest_size: Some(frame.dest_size),
            ..Default::default()
        },
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    // let dragon: Texture2D = load_texture("resources/dragon_drawchan.png").await.unwrap();
    let a: Texture2D = load_texture("resources/a.png").await.unwrap();
    let b: Texture2D = load_texture("resources/b.png").await.unwrap();
    let c: Texture2D = load_texture("resources/c.png").await.unwrap();
    let d: Texture2D = load_texture("resources/d.png").await.unwrap();
    let fog: Texture2D = load_texture("resources/fog.png").await.unwrap();
    let fog_half_transparent: Texture2D = load_texture("resources/fog_half_transparent.png")
        .await
        .unwrap();

    let spritepack = load_texture("resources/sprites_for_para.png")
        .await
        .unwrap();

    // let cat: Texture2D = load_texture("resources/cathead.png").await.unwrap();

    let lvl1_string = load_string("resources/level1.txt").await.unwrap();

    let lvl = Level::load_from_string(&lvl1_string);

    let world = World {
        dim: vec2(
            (lvl.width as f32) * TILE_SIZE,
            (lvl.height as f32) * TILE_SIZE,
        ),
    };

    let controls = Controls {
        is_up: false,
        is_down: false,
        is_left: false,
        is_right: false,
    };

    let player = Player {
        pos: vec2(
            (world.dim.x + TILE_SIZE) / 2.,
            (world.dim.y + TILE_SIZE) / 2.,
        ),
        dim: vec2(TILE_SIZE, TILE_SIZE),
        light_radius: 4,
        sprite: StaticSprite {
            frame: Frame {
                texture: &spritepack,
                source_rect: Rect {
                    x: HYOTTOKO_OFFSET,
                    y: 0.,
                    w: TILE_SIZE,
                    h: TILE_SIZE,
                },
                dest_size: Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                },
            },
        },
    };

    let mut off_x = 0.;
    let mut off_y = 0.;
    let mut cam_w = screen_width();
    let mut cam_h = screen_height();
    if cam_w > world.dim.x {
        off_x = (cam_w - world.dim.x) / 2.;
        cam_w = world.dim.x
    }
    if cam_h > world.dim.y {
        off_y = (cam_h - world.dim.y) / 2.;
        cam_h = world.dim.y
    }
    let camera = GameCamera {
        dim: vec2(cam_w, cam_h),
        pos: vec2(player.pos.x - cam_w / 2., player.pos.y - cam_h / 2.),
    };

    let mut game = Game {
        lvl,
        player,
        enemies: vec![],
        world,
        controls,
        camera,
    };

    game.add_enemy(create_blob(200., 200., &spritepack));
    game.add_enemy(create_blob(200., 300., &spritepack));
    game.add_enemy(create_blob(600., 200., &spritepack));
    game.add_enemy(create_blob(700., 500., &spritepack));
    game.add_enemy(create_spider(400., 200., &spritepack));
    game.add_enemy(create_spider(500., 200., &spritepack));
    game.add_enemy(create_spider(800., 200., &spritepack));
    game.add_enemy(create_spider(800., 500., &spritepack));

    loop {
        // draw everything
        clear_background(RED);

        let game_off_x = -game.camera.pos.x + off_x;
        let game_off_y = -game.camera.pos.y + off_y;

        for y in 0..game.lvl.height {
            for x in 0..game.lvl.width {
                let idx = y * game.lvl.width + x;
                let tile = &game.lvl.tiles[idx];
                match tile.fog {
                    FogLevel::HalfTransparent | FogLevel::Transparent => {
                        let tex = match tile.texture {
                            game::level::TileTexture::A => Some(&a),
                            game::level::TileTexture::B => Some(&b),
                            game::level::TileTexture::C => Some(&c),
                            game::level::TileTexture::D => Some(&d),
                            _ => None,
                        };
                        if let Some(tex) = tex {
                            draw_texture(
                                tex,
                                x as f32 * TILE_SIZE + game_off_x,
                                y as f32 * TILE_SIZE + game_off_y,
                                WHITE,
                            );
                        }
                    }
                    _ => {
                        // nothing
                    }
                }
            }
        }

        // draw enemies
        for enemy in &mut game.enemies {
            if !game.lvl.is_fog_of_war_at(enemy.pos) {
                draw_frame(
                    &enemy.sprite.frames[enemy.sprite.frame_index],
                    enemy.pos.x - (enemy.dim.x / 2.) + game_off_x,
                    enemy.pos.y - (enemy.dim.y / 2.) + game_off_y,
                );
            }
        }

        // draw player
        draw_frame(
            &game.player.sprite.frame,
            game.player.pos.x - (game.player.dim.x / 2.) + game_off_x,
            game.player.pos.y - (game.player.dim.y / 2.) + game_off_y,
        );

        // draw fog
        for y in 0..game.lvl.height {
            for x in 0..game.lvl.width {
                let idx = y * game.lvl.width + x;
                let tile = &game.lvl.tiles[idx];
                match tile.fog {
                    FogLevel::Opaque => {
                        draw_texture(
                            &fog,
                            x as f32 * TILE_SIZE + game_off_x,
                            y as f32 * TILE_SIZE + game_off_y,
                            WHITE,
                        );
                    }
                    FogLevel::HalfTransparent => {
                        draw_texture(
                            &fog_half_transparent,
                            x as f32 * TILE_SIZE + game_off_x,
                            y as f32 * TILE_SIZE + game_off_y,
                            WHITE,
                        );
                    }
                    _ => {
                        // nothing
                    }
                }
            }
        }

        // debug player position
        draw_text(
            format!("{} {}", game.player.pos.x, game.player.pos.y).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );

        // debug pressed direction keys
        if game.controls.is_right {
            draw_text(">", 60.0, 60.0, 30.0, ORANGE);
        }
        if game.controls.is_left {
            draw_text("<", 20.0, 60.0, 30.0, ORANGE);
        }
        if game.controls.is_up {
            draw_text("^", 40.0, 40.0, 30.0, ORANGE);
        }
        if game.controls.is_down {
            draw_text("V", 40.0, 60.0, 30.0, ORANGE);
        }

        // debug tiles in each direction of player tile
        let idx = game.lvl.tile_index_at(game.player.pos);
        if let Some(idx) = idx {
            if let Some(idx) = game.lvl.tile_index_above(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off_x,
                    pos.y * TILE_SIZE + game_off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    WHITE,
                );
            }
            if let Some(idx) = game.lvl.tile_index_below(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off_x,
                    pos.y * TILE_SIZE + game_off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    RED,
                );
            }
            if let Some(idx) = game.lvl.tile_index_left(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off_x,
                    pos.y * TILE_SIZE + game_off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLUE,
                );
            }
            if let Some(idx) = game.lvl.tile_index_right(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off_x,
                    pos.y * TILE_SIZE + game_off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLACK,
                );
            }
        }

        // handle input
        game.update();

        next_frame().await
    }
}
