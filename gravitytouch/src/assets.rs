use std::collections::HashMap;
use macroquad::prelude::*;

pub struct SpriteSheet {
    pub texture: Texture2D,
    pub frame_w: f32,
    pub frame_h: f32,
    pub cols: usize,
}

impl SpriteSheet {
    pub fn frame_rect(&self, frame_index: usize) -> Rect {
        let col = (frame_index % self.cols) as f32;
        let row = (frame_index / self.cols) as f32;
        Rect::new(col * self.frame_w, row * self.frame_h, self.frame_w, self.frame_h)
    }
}

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    sheets: HashMap<String, SpriteSheet>,
}

#[allow(dead_code)]
impl AssetManager {
    pub fn new() -> Self {
        AssetManager { textures: HashMap::new(), sheets: HashMap::new() }
    }

    pub async fn load(&mut self, name: &str, path: &str) -> Result<(), macroquad::Error> {
        let tex = load_texture(path).await?;
        self.textures.insert(name.to_string(), tex);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn register_sheet(&mut self, sheet_name: &str, texture_name: &str, frame_w: f32, frame_h: f32, cols: usize) {
        let tex = self.textures[texture_name].clone();
        self.sheets.insert(sheet_name.to_string(), SpriteSheet { texture: tex, frame_w, frame_h, cols });
    }

    pub fn get_sheet(&self, name: &str) -> Option<&SpriteSheet> {
        self.sheets.get(name)
    }
}

pub fn draw_sprite(texture: &Texture2D, src_rect: Rect, dest_x: f32, dest_y: f32, scale: f32) {
    draw_texture_ex(
        texture,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            source: Some(src_rect),
            dest_size: Some(Vec2::new(src_rect.w * scale, src_rect.h * scale)),
            ..Default::default()
        },
    );
}
