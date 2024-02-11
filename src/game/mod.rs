use macroquad::{
    math::{vec2, Rect, Vec2},
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

pub struct Game<'a> {
    pub lvl: Level,
    pub player: Player<'a>,
    pub enemies: Vec<Enemy<'a>>,
    pub world: World,
    pub controls: Controls,
    pub camera: GameCamera,
}

impl<'a> Game<'a> {
    pub fn add_enemy(&mut self, enemy: Enemy<'a>) {
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, res: &'a Resources) {
        self.update_controls();

        self.update_enemies();

        // TODO: meh .. put everything into update_player
        let time = get_time();
        if self.controls.is_q && !self.player.q.is_on_cooldown(time) {
            // launch rocket
            let mut rocket = create_enemy("Rocket".to_string(), self.player.pos, res);
            rocket.dir = (self.controls.mouse_pos - self.player.pos).normalize();
            self.add_enemy(rocket);
            self.player.q.last_use = Some(time);
        }

        let enemy_idx = self.get_enemy_index_at_pos(self.controls.mouse_pos);
        if self.controls.is_right_mouse_click && enemy_idx.is_some() {
            if !self.player.auto.is_on_cooldown(time) {
                let mut shot = create_enemy("Shot".to_string(), self.player.pos, res);
                shot.dir = (self.enemies[enemy_idx.unwrap()].pos - self.player.pos).normalize();
                self.add_enemy(shot);
                self.player.auto.last_use = Some(time);
            }
            self.player.target_pos = None
        } else {
            self.update_player()
        }

        self.update_level();
        self.update_camera();
    }

    pub fn get_enemy_index_at_pos(&self, pos: Vec2) -> Option<usize> {
        for i in 0..self.enemies.len() {
            let rect = Rect {
                x: self.enemies[i].pos.x - self.enemies[i].dim.x / 2.,
                y: self.enemies[i].pos.y - self.enemies[i].dim.y / 2.,
                w: self.enemies[i].dim.x,
                h: self.enemies[i].dim.y,
            };
            if rect.contains(pos) {
                return Some(i);
            }
        }
        None
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
