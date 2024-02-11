mod game;

use std::collections::HashMap;

use game::camera::GameCamera;
use game::controls::Controls;
use game::enemy::{create_enemy, Enemy, EnemyStrategy};
use game::gfx::{Frame, StaticSprite};
use game::inventory::Inventory;
use game::level::{FogLevel, Level, World, TILE_SIZE};
use game::player::{Ability, Player};
use game::resources::{
    load_enemy_definitions, load_level_definition, load_tile_definitions, Resources,
};
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

const HYOTTOKO_OFFSET: f32 = 64.;

fn draw_enemy(enemy: &Enemy, game_off: Vec2, is_in_fog: bool) {
    if !enemy.fog_of_war || !is_in_fog {
        draw_frame(
            &enemy.sprite.frames[enemy.sprite.frame_index],
            enemy.pos.x - (enemy.dim.x / 2.) + game_off.x,
            enemy.pos.y - (enemy.dim.y / 2.) + game_off.y,
        );
    }

    if !is_in_fog
        && !matches!(enemy.strategy, EnemyStrategy::Projectile)
        && enemy.hp != enemy.hp_max
    {
        // max hp red
        draw_rectangle(
            enemy.pos.x - (enemy.dim.x / 2.) + game_off.x,
            enemy.pos.y - (enemy.dim.y / 2.) - 10. + game_off.y,
            enemy.dim.x,
            5.,
            RED,
        );
        // current hp green
        draw_rectangle(
            enemy.pos.x - (enemy.dim.x / 2.) + game_off.x,
            enemy.pos.y - (enemy.dim.y / 2.) - 10. + game_off.y,
            enemy.dim.x * (enemy.hp as f32) / (enemy.hp_max as f32),
            5.,
            LIME,
        );
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

async fn init_level<'a>(path: &'a str, res: &'a Resources) -> Game<'a> {
    let lvl_def = load_level_definition(path).await;
    let lvl = Level::load_from_string(&(lvl_def.tiles.join("\n") + "\n"));
    let world = World {
        dim: vec2(
            (lvl.width as f32) * TILE_SIZE,
            (lvl.height as f32) * TILE_SIZE,
        ),
    };

    let controls = Controls {
        mouse_pos: vec2(0., 0.),
        is_left_mouse_click: false,
        is_right_mouse_click: false,
        is_q: false,
    };

    let player = Player {
        pos: vec2(lvl_def.player.x, lvl_def.player.y),
        dim: vec2(TILE_SIZE, TILE_SIZE),
        target_pos: None,
        speed: 3.,
        light_radius: 4,
        sprite: StaticSprite {
            frame: Frame {
                texture: res.textures.get("sprites_for_para.png").unwrap(),
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
        inventory: Inventory {
            amulet: None,
            head: None,
            body: None,
            arm1: None,
            arm2: None,
            ring1: None,
            ring2: None,
            belt: None,
            foot: None,
        },
        q: Ability {
            cooldown: 1.,
            last_use: None,
        },
        auto: Ability {
            cooldown: 0.5,
            last_use: None,
        },
    };

    let mut cam_w = screen_width();
    let mut cam_h = screen_height();
    if cam_w > world.dim.x {
        cam_w = world.dim.x
    }
    if cam_h > world.dim.y {
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

    for enemy in lvl_def.enemies {
        game.add_enemy(create_enemy(enemy.name, vec2(enemy.x, enemy.y), res));
    }

    game
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut res = Resources {
        enemy_definitions: load_enemy_definitions().await,
        tile_defintions: load_tile_definitions().await,
        textures: HashMap::new(),
    };

    res = res.load_textures().await;

    let fog: Texture2D = load_texture("resources/fog.png").await.unwrap();
    let fog_half_transparent: Texture2D = load_texture("resources/fog_half_transparent.png")
        .await
        .unwrap();

    let mut game = init_level("resources/level2.json", &res).await;

    loop {
        // draw everything
        clear_background(RED);

        let game_off = game.offset();

        for y in 0..game.lvl.height {
            for x in 0..game.lvl.width {
                let idx = y * game.lvl.width + x;
                let tile = &game.lvl.tiles[idx];
                match tile.fog {
                    FogLevel::HalfTransparent | FogLevel::Transparent => {
                        let tex = res.tile_texture(tile.ch);
                        if let Some(tex) = tex {
                            draw_texture(
                                tex,
                                x as f32 * TILE_SIZE + game_off.x,
                                y as f32 * TILE_SIZE + game_off.y,
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
            let is_in_fog = game.lvl.is_fog_of_war_at(enemy.pos);
            draw_enemy(enemy, game_off, is_in_fog)
        }

        // draw player
        draw_frame(
            &game.player.sprite.frame,
            game.player.pos.x - (game.player.dim.x / 2.) + game_off.x,
            game.player.pos.y - (game.player.dim.y / 2.) + game_off.y,
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
                            x as f32 * TILE_SIZE + game_off.x,
                            y as f32 * TILE_SIZE + game_off.y,
                            WHITE,
                        );
                    }
                    FogLevel::HalfTransparent => {
                        draw_texture(
                            &fog_half_transparent,
                            x as f32 * TILE_SIZE + game_off.x,
                            y as f32 * TILE_SIZE + game_off.y,
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
            format!("{:?}", game.player.pos).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );

        // debug mouse position
        draw_text(
            format!("{:?}", game.controls.mouse_pos).as_str(),
            20.0,
            40.0,
            30.0,
            DARKGRAY,
        );

        // debug enemy* count (incl. projectiles)
        draw_text(
            format!("{:?}", game.enemies.len()).as_str(),
            20.0,
            60.0,
            30.0,
            DARKGRAY,
        );

        if game.player.target_pos.is_some() {
            let target_pos = game.player.target_pos.unwrap();
            draw_line(
                game.player.pos.x + game_off.x,
                game.player.pos.y + game_off.y,
                target_pos.x + game_off.x,
                target_pos.y + game_off.y,
                1.0,
                ORANGE,
            )
        }

        // debug tiles in each direction of player tile
        let idx = game.lvl.tile_index_at(game.player.pos);
        if let Some(idx) = idx {
            if let Some(idx) = game.lvl.tile_index_above(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off.x,
                    pos.y * TILE_SIZE + game_off.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    WHITE,
                );
            }
            if let Some(idx) = game.lvl.tile_index_below(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off.x,
                    pos.y * TILE_SIZE + game_off.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    RED,
                );
            }
            if let Some(idx) = game.lvl.tile_index_left(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off.x,
                    pos.y * TILE_SIZE + game_off.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLUE,
                );
            }
            if let Some(idx) = game.lvl.tile_index_right(idx) {
                let pos = game.lvl.pos_by_index(idx);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE + game_off.x,
                    pos.y * TILE_SIZE + game_off.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLACK,
                );
            }
        }

        // handle input
        game.update(&res);

        next_frame().await
    }
}
