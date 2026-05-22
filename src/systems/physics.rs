use hecs::{Entity, World};
use macroquad::prelude::{screen_height, screen_width};

use crate::components::{Position, Velocity, Weight};

const G: f32 = 0.0001;  // gravitational constant in game units
const R_MIN: f32 = 0.05; // softening distance to avoid singularity at r → 0

pub fn system_physics(world: &mut World, dt: f32) {
    // Collect attractor state before taking mutable borrows on the world.
    let attractors: Vec<(f32, f32, f32)> = world
        .query::<(&Position, &Weight)>()
        .iter()
        .map(|(_, (pos, wt))| (pos.x, pos.y, wt.weight))
        .collect();

    for (_, (pos, vel)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
        // Newton: a = G * M / r²  directed toward
        for &(ax, ay, mass) in &attractors {
            let dx = ax - pos.x;
            let dy = ay - pos.y;
            let r = dx.hypot(dy).max(R_MIN);
            let acc = G * mass / (r * r);
            vel.dx += acc * (dx / r) * dt;
            vel.dy += acc * (dy / r) * dt;
        }

        pos.x += vel.dx * dt;
        pos.y += vel.dy * dt;
    }
}
