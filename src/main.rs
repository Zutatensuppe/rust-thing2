mod game;

use game::camera::GameCamera;
use game::controls::Controls;
use game::level::{Level, World, TILE_SIZE};
use game::player::Player;
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

#[macroquad::main(window_conf)]
async fn main() {
    // let dragon: Texture2D = load_texture("resources/dragon_drawchan.png").await.unwrap();
    let a: Texture2D = load_texture("resources/a.png").await.unwrap();
    let b: Texture2D = load_texture("resources/b.png").await.unwrap();
    let c: Texture2D = load_texture("resources/c.png").await.unwrap();
    let d: Texture2D = load_texture("resources/d.png").await.unwrap();
    let fog: Texture2D = load_texture("resources/fog.png").await.unwrap();

    let cat: Texture2D = load_texture("resources/cathead.png").await.unwrap();

    let lvl1_string = load_string("resources/level1.txt").await.unwrap();

    let mut lvl = Level::load_from_string(&lvl1_string);

    let world = World {
        dim: vec2(
            (lvl.width as f32) * TILE_SIZE,
            (lvl.height as f32) * TILE_SIZE,
        ),
    };

    let mut controls = Controls {
        is_up: false,
        is_down: false,
        is_left: false,
        is_right: false,
    };

    let mut player = Player {
        pos: vec2(
            (world.dim.x + TILE_SIZE) / 2.,
            (world.dim.y + TILE_SIZE) / 2.,
        ),
        dim: vec2(cat.width(), cat.height()),
        light_radius: 4,
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
    let mut camera = GameCamera {
        dim: vec2(cam_w, cam_h),
        pos: vec2(player.pos.x - cam_w / 2., player.pos.y - cam_h / 2.),
    };

    loop {
        // draw everything
        clear_background(RED);

        for y in 0..lvl.height {
            for x in 0..lvl.width {
                let idx = y * lvl.width + x;
                let tile = &lvl.tiles[idx];
                if tile.fog {
                    draw_texture(
                        &fog,
                        x as f32 * TILE_SIZE - camera.pos.x + off_x,
                        y as f32 * TILE_SIZE - camera.pos.y + off_y,
                        WHITE,
                    );
                } else {
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
                            x as f32 * TILE_SIZE - camera.pos.x + off_x,
                            y as f32 * TILE_SIZE - camera.pos.y + off_y,
                            WHITE,
                        );
                    }
                }
            }
        }

        draw_texture(
            &cat,
            player.pos.x - (player.dim.x / 2.) - camera.pos.x + off_x,
            player.pos.y - (player.dim.y / 2.) - camera.pos.y + off_y,
            WHITE,
        );

        // debug player position
        draw_text(
            format!("{} {}", player.pos.x, player.pos.y).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );

        // debug pressed direction keys
        if controls.is_right {
            draw_text(">", 60.0, 60.0, 30.0, ORANGE);
        }
        if controls.is_left {
            draw_text("<", 20.0, 60.0, 30.0, ORANGE);
        }
        if controls.is_up {
            draw_text("^", 40.0, 40.0, 30.0, ORANGE);
        }
        if controls.is_down {
            draw_text("V", 40.0, 60.0, 30.0, ORANGE);
        }

        // debug tiles in each direction of player tile
        let idx = lvl.tile_index_at(player.pos);
        if let Some(idx) = idx {
            let above = lvl.tile_index_above(idx);
            let below = lvl.tile_index_below(idx);
            let left = lvl.tile_index_left(idx);
            let right = lvl.tile_index_right(idx);
            if let Some(above) = above {
                let pos = lvl.pos_by_index(above);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE - camera.pos.x + off_x,
                    pos.y * TILE_SIZE - camera.pos.y + off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    WHITE,
                );
            }
            if let Some(below) = below {
                let pos = lvl.pos_by_index(below);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE - camera.pos.x + off_x,
                    pos.y * TILE_SIZE - camera.pos.y + off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    RED,
                );
            }
            if let Some(left) = left {
                let pos = lvl.pos_by_index(left);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE - camera.pos.x + off_x,
                    pos.y * TILE_SIZE - camera.pos.y + off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLUE,
                );
            }
            if let Some(right) = right {
                let pos = lvl.pos_by_index(right);
                draw_rectangle_lines(
                    pos.x * TILE_SIZE - camera.pos.x + off_x,
                    pos.y * TILE_SIZE - camera.pos.y + off_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    3.,
                    BLACK,
                );
            }
        }

        // handle input
        controls = controls.update();

        // update
        player = player.update(&controls, &lvl);
        lvl = lvl.update(&player);
        camera = camera.update(&world, &player);

        next_frame().await
    }
}
