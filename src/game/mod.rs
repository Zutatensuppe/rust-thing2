use macroquad::{
    math::{vec2, Vec2},
    time::get_time,
    window::{screen_height, screen_width},
};

use self::{
    camera::GameCamera,
    controls::Controls,
    enemy::{create_enemy, Enemy},
    level::{Level, World},
    player::Player,
    resources::Resources,
};

pub mod camera;
pub mod controls;
pub mod enemy;
pub mod entity;
pub mod gfx;
pub mod inventory;
pub mod level;
pub mod player;
pub mod resources;
pub mod util;

pub struct Game<'a> {
    pub lvl: Level,
    pub player: Player<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub world: World,
    pub controls: Controls,
    pub camera: GameCamera,
}

impl<'a> Game<'a> {
    pub fn add_enemy(&mut self, mut enemy: Enemy<'a>) {
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, res: &'a Resources) {
        self.update_controls();

        self.update_enemies();
        self.update_player();
        self.update_level();
        self.update_camera();

        if self.controls.is_q {
            let time = get_time();
            if !self.player.q.is_on_cooldown(time) {
                // launch rocket
                let mut rocket = create_enemy("Rocket".to_string(), self.player.pos, res);
                rocket.dir = (self.controls.mouse_pos - self.player.pos).normalize();
                self.add_enemy(rocket);
                self.player.q.last_use = Some(time);
            }
        }
    }

    pub fn offset(&self) -> Vec2 {
        let mut off_x = 0.;
        let mut off_y = 0.;
        let cam_w = screen_width();
        let cam_h = screen_height();
        if cam_w > self.world.dim.x {
            off_x = (cam_w - self.world.dim.x) / 2.;
        }
        if cam_h > self.world.dim.y {
            off_y = (cam_h - self.world.dim.y) / 2.;
        }

        let game_off_x = -self.camera.pos.x + off_x;
        let game_off_y = -self.camera.pos.y + off_y;
        vec2(game_off_x, game_off_y)
    }
}
