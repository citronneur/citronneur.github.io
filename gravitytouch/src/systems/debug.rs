use hecs::World;
use macroquad::math::Vec2;
use macroquad::prelude::*;

use crate::components::{DebugTag, Geometry, Position, Transformation};

pub fn system_debug(world: &World) {
    if let Some((_, debug)) = world.query::<&mut DebugTag>().into_iter().next() {
        if !debug.print_geometry {
            return;
        }
    }

    for (entity, (pos, geo, tf)) in world
        .query::<(&Position, &Geometry, Option<&Transformation>)>()
        .iter()
    {
        match geo {
            Geometry::Circle(r) => {
                draw_circle_lines(pos.x, pos.y, *r, 1.0, GREEN);
                draw_text(
                    &format!("{:?} r={:.1}", entity, r),
                    pos.x + r + 4.0,
                    pos.y + 4.0,
                    14.0,
                    GREEN,
                );
            }
            Geometry::Rectangle(w, h) => {
                let hw = w * 0.5;
                let hh = h * 0.5;
                let local = [
                    Vec2::new(-hw, -hh),
                    Vec2::new( hw, -hh),
                    Vec2::new( hw,  hh),
                    Vec2::new(-hw,  hh),
                ];
                let mat = tf.map(|t| t.default * t.transformation).unwrap_or(glam::Mat2::IDENTITY);
                let corners: Vec<Vec2> = local
                    .iter()
                    .map(|&v| {
                        let r = mat * v;
                        Vec2::new(pos.x + r.x, pos.y + r.y)
                    })
                    .collect();
                for i in 0..4 {
                    let a = corners[i];
                    let b = corners[(i + 1) % 4];
                    draw_line(a.x, a.y, b.x, b.y, 1.0, GREEN);
                }
                draw_text(
                    &format!("{:?} {:.0}x{:.0}", entity, w, h),
                    corners[1].x + 4.0,
                    corners[1].y,
                    14.0,
                    GREEN,
                );
            }
        }
    }

    let fps = get_fps();
    draw_text(&format!("FPS: {fps}"), 10.0, 20.0, 20.0, GRAY);
    draw_text("ESC \u{2013} back to menu", 10.0, 68.0, 20.0, GRAY);
}
