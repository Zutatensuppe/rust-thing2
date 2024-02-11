mod game;

use std::collections::HashMap;

use game::camera::GameCamera;
use game::controls::Controls;
use game::enemy::{count_nexus, create_enemy, Enemy, EnemyStrategy};
use game::gfx::{Frame, StaticSprite};
use game::inventory::Inventory;
use game::level::{FogLevel, Level, World, TILE_SIZE};
use game::player::{Ability, Player};
use game::resources::{
    load_enemy_definitions, load_level_definition, load_tile_definitions, Resources,
};
use game::{Game, GameState, GameStats};
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

fn draw_ability(x: f32, y: f32, ability: &Ability) {
    let cooldown_left = ability.cooldown_left(get_time());
    if cooldown_left > 0.0 {
        draw_rectangle(x, y, 48., 48., GRAY);
        draw_rectangle(x + 2., y + 2., 44., 44., LIGHTGRAY);
        draw_text(ability.name.as_str(), x + 2., y + 2. + 20.0, 30.0, BLACK);
        draw_text(
            format!("{:.1$}", cooldown_left, 2).as_str(),
            x + 2.,
            y + 2. + 40.0,
            20.0,
            BLACK,
        );
    } else {
        draw_rectangle(x, y, 48., 48., PINK);
        draw_rectangle(x + 2., y + 2., 44., 44., YELLOW);
        draw_text(ability.name.as_str(), x + 2., y + 2. + 20.0, 30.0, RED);
        draw_text("READY", x + 2., y + 2. + 40.0, 20.0, BLUE);
    }
}

fn draw_game(
    game: &Game,
    game_off: Vec2,
    fog: &Texture2D,
    fog_half_transparent: &Texture2D,
    res: &Resources,
) {
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
    for enemy in &game.enemies {
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
                        fog,
                        x as f32 * TILE_SIZE + game_off.x,
                        y as f32 * TILE_SIZE + game_off.y,
                        WHITE,
                    );
                }
                FogLevel::HalfTransparent => {
                    draw_texture(
                        fog_half_transparent,
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
}

fn draw_hud(game: &Game) {
    let hud_height = 64.0;
    draw_rectangle(
        0.0,
        screen_height() - hud_height,
        screen_width(),
        hud_height,
        GRAY,
    );

    let border = 8.;
    let x = border;
    let y = screen_height() - hud_height + border;

    // player HP
    let bar_height = 12.;
    draw_rectangle(x, y, 96., bar_height, DARKGRAY);
    draw_rectangle(
        x,
        y,
        96. * (game.player.hp as f32) / (game.player.hp_max as f32),
        bar_height,
        GOLD,
    );
    draw_text(
        format!("{:?}/{:?}", game.player.hp, game.player.hp_max).as_str(),
        x,
        y + 11.0,
        20.0,
        BLACK,
    );

    // abilities
    let mut x = 128. + border;
    draw_ability(x, y, &game.player.auto);
    x += 48.0 + border;
    draw_ability(x, y, &game.player.q);
}

fn draw_debug(game: &Game, game_off: Vec2) {
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

    // debug player target position
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
        hp: 100,
        hp_max: 100,
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
            cooldown: 4.,
            last_use: None,
            name: "Q".to_string(),
        },
        auto: Ability {
            cooldown: 0.7,
            last_use: None,
            name: "AUTO".to_string(),
        },
    };

    let mut cam_w = screen_width();
    let mut cam_h = screen_height() - 64.;
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
        stats: GameStats {
            damage_dealt: 0,
            damage_received: 0,
            enemies_killed: 0,
            time_spent: 0.,
        },
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

    let mut state = GameState::MainMenu;

    let levels = ["resources/level1.json", "resources/level2.json"];
    let mut level_idx = 0;
    let mut option_game = None;

    let mut mouse_down = false;
    let mut last_time = None;

    loop {
        let time = get_time();
        if option_game.is_none() {
            option_game = Some(init_level(levels[level_idx], &res).await);
        }

        let game = option_game.as_mut().unwrap();
        let game_off = game.offset();

        // clear everything
        clear_background(RED);

        draw_game(game, game_off, &fog, &fog_half_transparent, &res);

        match state {
            GameState::InGame => {
                if last_time.is_some() {
                    game.stats.time_spent += time - last_time.unwrap();
                }
                last_time = Some(time);

                // hud
                draw_hud(game);

                // debug stuff
                draw_debug(game, game_off);

                game.update(&res);
                if count_nexus(&game.enemies) == 0 || game.player.hp == 0 {
                    state = GameState::PostGame;
                    mouse_down = false;
                }
            }
            GameState::PostGame => {
                let victory = count_nexus(&game.enemies) == 0;
                let text;
                let color;
                if victory {
                    text = "VICTORY!!!";
                    color = LIME;
                } else {
                    text = "DEFEAT!!!";
                    color = RED;
                }
                let font_size = 100;
                let size = measure_text(text, None, font_size, 1.0);
                let x = (screen_width() - size.width) / 2.;
                let mut y = (screen_height() - size.height) / 2. + size.offset_y;
                draw_text(text, x, y, 100.0, color);
                y += 60.;
                let text = format!("TIME: {:.1$}SEC", game.stats.time_spent, 2);
                draw_text(text.as_str(), x, y, 30.0, WHITE);
                y += 30.;

                let text = format!("ENEMIES SLAIN: {:?}", game.stats.enemies_killed);
                draw_text(text.as_str(), x, y, 30.0, WHITE);
                y += 30.;

                let text = format!("DAMAGE DEALT: {:?}", game.stats.damage_dealt);
                draw_text(text.as_str(), x, y, 30.0, WHITE);
                y += 30.;

                let text = format!("DAMAGE RECEIVED: {:?}", game.stats.damage_received);
                draw_text(text.as_str(), x, y, 30.0, WHITE);
                y += 60.;

                let text = match victory {
                    true => "CLICK TO CONTINUE",
                    false => "CLICK TO TRY AGAIN",
                };
                draw_text(text, x, y, 30.0, WHITE);

                if is_mouse_button_down(MouseButton::Left) {
                    mouse_down = true
                } else if mouse_down {
                    if victory {
                        level_idx += 1;
                    }
                    if level_idx >= levels.len() {
                        state = GameState::MainMenu;
                    } else {
                        option_game = None;
                        state = GameState::InGame;
                    }
                }
            }
            GameState::MainMenu => {
                level_idx = 0;
                option_game = None;
                state = GameState::InGame;
            }
        };

        next_frame().await
    }
}
