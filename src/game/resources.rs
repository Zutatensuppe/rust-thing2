use std::collections::HashMap;

use macroquad::{
    file::load_string,
    texture::{load_texture, Texture2D},
};
use serde::{Deserialize, Serialize};

// like Vec2, only needed for de/serialization
#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct EnemySpriteDefinition {
    pub texture: String,
    pub frames: Vec<Point>,
    pub fps: u8,
}

#[derive(Serialize, Deserialize)]
pub struct EnemyDefinition {
    pub name: String,
    pub strategy: String,
    pub speed: f32,
    pub speed_solid: f32,
    pub sprite: EnemySpriteDefinition,
    pub dim: Point,
    pub fog_of_war: bool,
    pub hp_max: usize,
}

#[derive(Serialize, Deserialize)]
pub struct TileDefinition {
    pub name: String,
    pub ch: char,
    pub texture: String,
    pub solid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LevelEnemyDefinition {
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct LevelDefinition {
    pub tiles: Vec<String>,
    pub player: Point,
    pub enemies: Vec<LevelEnemyDefinition>,
}

pub struct Resources {
    pub enemy_definitions: HashMap<String, EnemyDefinition>,
    pub tile_defintions: HashMap<char, TileDefinition>,
    pub textures: HashMap<String, Texture2D>,
}

pub async fn load_enemy_definitions() -> HashMap<String, EnemyDefinition> {
    let json_string = load_string("resources/enemies.json").await.unwrap();
    let defs: Vec<EnemyDefinition> = serde_json::from_str(&json_string).unwrap();
    defs.into_iter()
        .map(|def| (def.name.to_string(), def))
        .collect::<HashMap<String, EnemyDefinition>>()
}

pub async fn load_tile_definitions() -> HashMap<char, TileDefinition> {
    let json_string = load_string("resources/tiles.json").await.unwrap();
    let defs: Vec<TileDefinition> = serde_json::from_str(&json_string).unwrap();
    defs.into_iter()
        .map(|def| (def.ch, def))
        .collect::<HashMap<char, TileDefinition>>()
}

pub async fn load_level_definition(path: &str) -> LevelDefinition {
    let json_string = load_string(path).await.unwrap();
    serde_json::from_str(&json_string).unwrap()
}

impl Resources {
    pub async fn load_textures(mut self) -> Self {
        for (_, def) in &self.enemy_definitions {
            if self.textures.contains_key(&def.sprite.texture) {
                continue;
            }
            let path = format!("resources/textures/{}", def.sprite.texture);
            let key = def.sprite.texture.to_string();
            let val = load_texture(&path).await.unwrap();
            self.textures.insert(key, val);
        }
        for (_, def) in &self.tile_defintions {
            if self.textures.contains_key(&def.texture) {
                continue;
            }
            let path = format!("resources/textures/{}", def.texture);
            let key = def.texture.to_string();
            let val = load_texture(&path).await.unwrap();
            self.textures.insert(key, val);
        }
        self
    }

    pub fn tile_texture(&self, ch: char) -> Option<&Texture2D> {
        let def = self.tile_defintions.get(&ch);
        if let Some(def) = def {
            return Some(&self.textures[&def.texture]);
        }
        None
    }
}
