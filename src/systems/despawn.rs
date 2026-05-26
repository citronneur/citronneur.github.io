use hecs::World;
use macroquad::prelude::{screen_height, screen_width};

use crate::components::{Geometry, Position, Velocity};

pub fn system_despawn(world: &mut World) {
    let sw = screen_width();
    let sh = screen_height();

    let to_remove: Vec<hecs::Entity> = world
        .query::<(&Position, &Geometry, &Velocity)>()
        .iter()
        .filter_map(|(entity, (pos, geo, _))| {
            let outside = match geo {
                Geometry::Circle(r) => {
                    pos.x + r < 0.0 || pos.x - r > sw || pos.y + r < 0.0 || pos.y - r > sh
                },
                Geometry::Rectangle(w, h) => {
                    false
                }
            };
            outside.then_some(entity)
        })
        .collect();

    for entity in to_remove {
        let _ = world.despawn(entity);
    }
}
