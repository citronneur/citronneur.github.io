use std::collections::{HashMap, HashSet};

use hecs::{Entity, World};
use macroquad::prelude::Color;

use crate::components::{CollideTag, GameObject, Geometry, Position, Velocity};

pub fn system_collide(world: &mut World) {

    // clear collision
    for (_, (col)) in world.query_mut::<(&mut CollideTag)>() {
        col.other = None;
    }

    let objects: Vec<(Entity, Position, Geometry)> = world
        .query::<(&Position, &Geometry, &CollideTag)>()// only collision entity
        .iter()
        .map(|(e, (p, g, _))| (e, p.clone(), g.clone()))
        .collect();

    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let (e1, p1, g1) = &objects[i];
            let (e2, p2, g2) = &objects[j];

            if !is_collide(p1, g1, p2, g2) {
                continue
            }

            let mut o1= world.query_one_mut::<&mut CollideTag>(*e1).unwrap();
            *o1 = CollideTag{other: Some(*e2)};
            let mut o2 = world.query_one_mut::<&mut CollideTag>(*e2).unwrap();
            *o2 = CollideTag{other: Some(*e2)};

        }
    }
}

pub fn is_collide(position_1: &Position, geometry_1: &Geometry, position_2: &Position, geometry_2: &Geometry) -> bool {
    match (geometry_1, geometry_2) {
        (Geometry::Circle(r1), Geometry::Circle(r2)) => {
            let dist = ((position_2.x - position_1.x) * (position_2.x - position_1.x) + (position_2.y - position_1.y) * (position_2.y - position_1.y)).sqrt();
            if dist >= r1 + r2 {
                false
            }
            else {
                true
            }
        }
        _ => panic!("Geometry collision is not handled"),
    }
}
