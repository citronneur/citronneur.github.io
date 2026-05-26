use std::f32::consts::PI;
use hecs::World;
use macroquad::audio::Sound;
use macroquad::color::Color;
use macroquad::math::Mat2;
use macroquad::prelude::{screen_height, screen_width};
use crate::assets::AssetManager;
use crate::components::{BlackHole, CollideTag, GameObject, Geometry, LevelManager, LevelState, PhysicsClock, Position, SpacecraftTag, Sprite, TargetTag, Transformation, Velocity, Weight};
use crate::scene::SceneTag;
use crate::systems::audio::system_audio;
use crate::systems::blackhole::system_blackhole;
use crate::systems::collision::system_collide;
use crate::systems::debug::system_debug;
use crate::systems::keyboard::system_keyboard;
use crate::systems::logic::system_logic;
use crate::systems::orientation::system_orientation;
use crate::systems::physics::system_physics;
use crate::systems::render::{system_render, BlackHoleRenderer};
use crate::systems::sprite_render::system_sprite_render;
use crate::systems::despawn::system_despawn;
use crate::systems::time::system_time;

const FIXED_DT: f32 = 1.0 / 60.0;
const MAX_STEPS: u32 = 8; // spiral-of-death guard


pub fn system_level(world: &mut World, assets: &AssetManager, sound: &Sound, dt: f32, bh_renderer: &mut Option<BlackHoleRenderer>) {
    let state = {
        let q = world.query_mut::<&LevelManager>();
        if let Some((_, manager)) = q.into_iter().next() {
            manager.state.clone()
        } else {
            LevelState::Running
        }
    };

    match state {
        LevelState::Running => system_level_running(world, assets, sound, dt, bh_renderer),
        LevelState::Translating => system_level_translating(world, assets, bh_renderer),
        LevelState::Spawn(n) => system_level_spawn(world, assets, bh_renderer, n)
    }
}
fn system_level_running(world: &mut World, assets: &AssetManager, sound: &Sound, dt: f32, bh_renderer: &mut Option<BlackHoleRenderer>) {
    system_time(world, dt);
    system_keyboard(world);
    system_blackhole(world, dt);

    let steps = {
        let q = world.query_mut::<&PhysicsClock>();
        if let Some((_, clock)) = q.into_iter().next() {
            clock.steps
        } else {
            0
        }
    };

    // Fixed-step loop: each tick is exactly FIXED_DT seconds.
    for _ in 0..steps {
        system_physics(world, FIXED_DT);
        system_collide(world);
    }

    system_despawn(world);
    system_logic(world);
    system_audio(world, sound);
    system_orientation(world);

    let density = {
        let mut q = world.query::<(&LevelManager)>();
        if let Some((_, (manager))) = q.into_iter().next() {
            manager.density
        }
        else {
            panic!("manager not found")
        }
    };

    system_render(world, bh_renderer, density);
    system_sprite_render(world, assets);
    system_debug(world);
}

fn system_level_translating(world: &mut World, assets: &AssetManager, bh_renderer: &mut Option<BlackHoleRenderer>) {
    let mut  is_end_translating = false;

    let mut target_pos = {
        let q = world.query_mut::<(&Position, &TargetTag)>();
        if let Some((_, (pos, _))) = q.into_iter().next() {
            pos.clone()
        }
        else {
            panic!("Target not found")
        }
    };

    let mut spacecraft_pos = {
        let q = world.query_mut::<(&Position, &SpacecraftTag)>();
        if let Some((_, (pos, _))) = q.into_iter().next() {
            pos.clone()
        }
        else {
            panic!("Target not found")
        }
    };
    let sw = screen_width();
    let sh = screen_height();
    let offset = Position { x: (sw - 1024.0)/2.0, y: (sh - 460.0) / 2.0};
    let origin = Position { x : 50.0 + offset.x, y : 230.0 + offset.y};

    let mut intesity_vector = Position { x :  spacecraft_pos.x - target_pos.x , y : spacecraft_pos.y - target_pos.y};
    let intensity = (intesity_vector.x * intesity_vector.x + intesity_vector.y * intesity_vector.y).sqrt();

    for (_, (pos, vel, obj)) in world.query_mut::<(&mut Position, &mut Velocity, &GameObject)>() {
        let mut direction = Position { x :  pos.x - target_pos.x , y : pos.y - target_pos.y};
        let norm = (direction.x * direction.x + direction.y * direction.y).sqrt();
        direction.x = direction.x / norm;
        direction.y = direction.y / norm;

        pos.x += direction.x * 300.0_f32.powf(intensity / 2024.0 + 0.1);
        pos.y += direction.y * 300.0_f32.powf(intensity / 2024.0 + 0.1);
        if *obj == GameObject::Airship {
            if (pos.x < origin.x + 50.0 || pos.x > origin.x + 1024.0) {
                is_end_translating = true;
            }
        }
    }

    if (is_end_translating) {
        // remove all black hole
        let to_remove: Vec<_> = world.query::<&BlackHole>().iter().map(|(e, _)| e).collect();
        for entity in to_remove {
            world.despawn(entity).ok();
        }

        let q = world.query_mut::<&mut LevelManager>();
        if let Some((_, manager)) = q.into_iter().next() {
            manager.state = LevelState::Running;
        }
    }

    let density = {
        let q = world.query_mut::<(&mut LevelManager)>();
        if let Some((_, (manager))) = q.into_iter().next() {
            manager.density -= 0.10;
            manager.density
        }
        else {
            panic!("manager not found")
        }
    };

    system_render(world, bh_renderer, density);
    system_sprite_render(world, assets);
}

fn system_level_spawn(world: &mut World, assets: &AssetManager, bh_renderer: &mut Option<BlackHoleRenderer>, stage: u32) {
    let mut  is_end_translating = false;
}