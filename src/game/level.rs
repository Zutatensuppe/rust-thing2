use macroquad::prelude::*;

use super::player::Player;

pub struct World {
    pub dim: Vec2,
}

pub const TILE_SIZE: f32 = 32.;

pub enum TileTexture {
    None,
    A,
    B,
    C,
    D,
}

impl TileTexture {
    fn from_char(ch: char) -> Self {
        match ch {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            _ => Self::None,
        }
    }
}

pub struct Tile {
    pub fog: bool,
    pub solid: bool,
    pub texture: TileTexture,
}

impl Tile {
    fn from_char(ch: char) -> Self {
        Self {
            fog: true,
            solid: !matches!(ch, 'c' | 'd'),
            texture: TileTexture::from_char(ch),
        }
    }
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Level {
    // Loads a level from a string
    // all lines in the string need to be of the same length for this to
    // work correctly right now
    // unknown characters in the string will result in empty tiles
    pub fn load_from_string(level_string: &str) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        let mut height = 0;
        for ch in level_string.chars() {
            match ch {
                '\n' => height += 1,
                _ => tiles.push(Tile::from_char(ch)),
            }
        }
        Level {
            width: tiles.len() / height,
            height,
            tiles,
        }
    }

    pub fn is_solid_at(&self, pos: Vec2) -> bool {
        let index = self.tile_index_at(pos);
        if let Some(index) = index {
            return self.tiles[index].solid;
        }

        false
    }

    pub fn tile_index_at(&self, pos: Vec2) -> Option<usize> {
        let x_index = pos.x as usize / TILE_SIZE as usize;
        let y_index = pos.y as usize / TILE_SIZE as usize;

        let index = y_index * self.width + x_index;
        if index < self.tiles.len() {
            return Some(index);
        }
        None
    }

    pub fn tile_index_above(&self, tile_index: usize) -> Option<usize> {
        if tile_index >= self.width {
            return Some(tile_index - self.width);
        }

        None
    }

    pub fn tile_index_below(&self, tile_index: usize) -> Option<usize> {
        if tile_index + self.width < self.tiles.len() {
            return Some(tile_index + self.width);
        }

        None
    }

    pub fn tile_index_left(&self, tile_index: usize) -> Option<usize> {
        if tile_index % self.width != 0 {
            return Some(tile_index - 1);
        }

        None
    }

    pub fn tile_index_right(&self, tile_index: usize) -> Option<usize> {
        if (tile_index + 1) % self.width != 0 {
            return Some(tile_index + 1);
        }

        None
    }

    pub fn pos_by_index(&self, tile_index: usize) -> Vec2 {
        let y = tile_index / self.width;
        let x = tile_index % self.width;
        vec2(x as f32, y as f32)
    }

    pub fn update(mut self, player: &Player) -> Self {
        // reveal fog of war around the player (in a very weird way :/)
        let player_tile_index = self.tile_index_at(player.pos);
        if let Some(player_tile_index) = player_tile_index {
            let player_tile_pos = self.pos_by_index(player_tile_index);
            for i in 0..self.tiles.len() {
                let tile_pos = self.pos_by_index(i);
                if player_tile_pos.distance(tile_pos) < player.light_radius as f32 {
                    self.tiles[i].fog = false;
                }
            }
        }
        self
    }
}
