use macroquad::{
    math::{Rect, Vec2},
    texture::Texture2D,
    time::get_frame_time,
};

pub struct StaticSprite<'a> {
    pub frame: Frame<'a>,
}

pub struct AnimatedSprite<'a> {
    pub frames: Vec<Frame<'a>>,
    pub frame_index: usize,
    pub fps: u8,
    pub time: f32,
}

impl<'a> AnimatedSprite<'a> {
    pub fn update(&mut self) {
        self.time += get_frame_time();
        if self.time > 1. / self.fps as f32 {
            self.frame_index += 1;
            self.time = 0.0;
        }
        self.frame_index %= self.frames.len();
    }
}

pub struct Frame<'a> {
    pub texture: &'a Texture2D,
    pub source_rect: Rect, // source rect in the texture
    pub dest_size: Vec2,   // drawing size
}
