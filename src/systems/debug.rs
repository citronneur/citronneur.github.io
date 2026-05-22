use hecs::World;
use macroquad::prelude::*;

use crate::components::{DebugTag, Geometry, Position};
pub fn system_debug(world: & World) {

    if let Some((_, debug)) = world.query::<&mut DebugTag>().into_iter().next() {
        if !debug.print_geometry {
            return;
        }
    }

    let sh = screen_height();
    for (entity, (pos, geo)) in world.query::<(&Position, &Geometry)>().iter() {
        match geo {
            Geometry::Circle(r) => {
                draw_circle_lines(pos.x * sh, pos.y * sh, *r * sh, 1.0, GREEN);
                draw_text(
                    &format!("{:?} r={:.1}", entity, r),
                    pos.x * sh + r + 4.0,
                    pos.y * sh + 4.0,
                    14.0,
                    GREEN,
                );
            }
        }
    }
}
