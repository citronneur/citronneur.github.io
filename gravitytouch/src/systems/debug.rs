use hecs::World;
use macroquad::prelude::*;

use crate::components::{DebugTag, Geometry, Position};
pub fn system_debug(world: & World) {

    if let Some((_, debug)) = world.query::<&mut DebugTag>().into_iter().next() {
        if !debug.print_geometry {
            return;
        }
    }

    for (entity, (pos, geo)) in world.query::<(&Position, &Geometry)>().iter() {
        match geo {
            Geometry::Circle(r) => {
                draw_circle_lines(pos.x, pos.y , *r, 1.0, GREEN);
                draw_text(
                    &format!("{:?} r={:.1}", entity, r),
                    pos.x + r + 4.0,
                    pos.y + 4.0,
                    14.0,
                    GREEN,
                );
            }
        }
    }

    // HUD
    let fps = get_fps();
    draw_text(&format!("FPS: {fps}"), 10.0, 20.0, 20.0, GRAY);
    draw_text("ESC \u{2013} back to menu", 10.0, 68.0, 20.0, GRAY);
}
