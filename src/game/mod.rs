use macroquad::math::Vec2;

use self::{
    camera::GameCamera,
    controls::Controls,
    enemy::Enemy,
    level::{Level, World},
    player::Player,
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
    pub fn add_enemy(&mut self, mut enemy: Enemy<'a>) {
        self.enemies.push(enemy);
    }

    pub fn update(&mut self) {
        self.update_controls();

        self.update_enemies();
        self.update_player();
        self.update_level();
        self.update_camera();
    }
}
