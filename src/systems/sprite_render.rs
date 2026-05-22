use hecs::World;
use macroquad::math::{Vec2, Vec3};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use macroquad::window::screen_height;

use crate::assets::{draw_sprite, AssetManager};
use crate::components::{PhysicsClock, Position, Sprite, Transformation};

pub fn system_sprite_render(world: &World, assets: &AssetManager) {
    let sh = screen_height();

    let global = if let Some((_, clock)) = world.query::<&mut PhysicsClock>().into_iter().next() {
        clock.global
    }
    else {
        0.0
    };


    // Entities with Transformation: apply Mat2 to the four corners and draw as a mesh.
    for (_, (pos, sprite, tf)) in world.query::<(&Position, &Sprite, &Transformation)>().iter() {
        if let Some(sheet) = assets.get_sheet(&sprite.sheet_name) {
            let src = sheet.frame_rect((global * 8.0) as usize  % sheet.cols);
            let cx = pos.x * sh;
            let cy = pos.y * sh;
            let hw = src.w * sh * sprite.scale * 0.5;
            let hh = src.h * sh * sprite.scale * 0.5;

            let tw = sheet.texture.width();
            let th = sheet.texture.height();
            let u0 = src.x / tw;
            let u1 = (src.x + src.w) / tw;
            let v0 = src.y / th;
            let v1 = (src.y + src.h) / th;

            let vert = |lx: f32, ly: f32, u: f32, v: f32| -> Vertex {
                let p = tf.default * tf.transformation * Vec2::new(lx, ly);
                Vertex {
                    position: Vec3::new(cx + p.x, cy + p.y, 0.0),
                    uv: Vec2::new(u, v),
                    normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
                    color: WHITE.into(),
                }
            };

            draw_mesh(&Mesh {
                vertices: vec![
                    vert(-hw, -hh, u0, v0),
                    vert( hw, -hh, u1, v0),
                    vert( hw,  hh, u1, v1),
                    vert(-hw,  hh, u0, v1),
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
                texture: Some(sheet.texture.clone()),
            });
        }
    }

    // Entities without Transformation: plain draw (no matrix applied).
    for (_, (pos, sprite)) in world.query::<(&Position, &Sprite)>().without::<&Transformation>().iter() {
        if let Some(sheet) = assets.get_sheet(&sprite.sheet_name) {
            let src = sheet.frame_rect((global * 8.0) as usize  % sheet.cols);
            draw_sprite(&sheet.texture, src, pos.x * sh, pos.y * sh, sprite.scale);
        }
    }
}
