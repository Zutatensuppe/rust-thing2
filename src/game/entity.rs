use macroquad::math::{Rect, Vec2};

use super::level::{Level, TILE_SIZE};

pub trait Entity {
    fn pos(&self) -> Vec2;
    fn dim(&self) -> Vec2;
}

pub fn collides(entity1: &dyn Entity, entity2: &dyn Entity) -> bool {
    let pos1 = entity1.pos();
    let dim1 = entity1.dim();
    let pos2 = entity2.pos();
    let dim2 = entity2.dim();

    let rect1 = Rect {
        x: pos1.x - (dim1.x / 2.),
        y: pos1.y - (dim1.y / 2.),
        w: dim1.x,
        h: dim1.y,
    };
    let rect2 = Rect {
        x: pos2.x - (dim2.x / 2.),
        y: pos2.y - (dim2.y / 2.),
        w: dim2.x,
        h: dim2.y,
    };

    rect1.overlaps(&rect2)
}

pub fn is_out_of_lvl_bounds(entity: &dyn Entity, lvl: &Level) -> bool {
    let pos = entity.pos();
    pos.x < 0.
        || pos.y < 0.
        || pos.x > (lvl.width as f32) * TILE_SIZE
        || pos.y > (lvl.height as f32) * TILE_SIZE
}
