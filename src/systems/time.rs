use hecs::World;
use crate::components::PhysicsClock;

pub const FIXED_DT: f32 = 1.0 / 60.0;
pub fn system_time(world: &mut World, dt: f32) {

    let q = world.query_mut::<&mut PhysicsClock>();
    if let Some((_, clock)) = q.into_iter().next() {
        clock.accumulator += dt;
        clock.global += dt;
        clock.steps = (clock.accumulator / FIXED_DT) as u32;
        clock.accumulator = dt - clock.steps as f32 * FIXED_DT;
    }
}