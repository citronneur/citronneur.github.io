use hecs::World;
use macroquad::audio::Sound;
use macroquad::prelude::*;

use crate::assets::AssetManager;
use crate::components::{PhysicsClock};
use crate::scene::{clear_scene, spawn_scene, SceneKind, SceneManager};
use crate::systems::{
    audio::system_audio,
    blackhole::system_blackhole,
    collision::system_collide,
    debug::system_debug,
    menu_input::system_menu_input,
    menu_render::system_menu_render,
    physics::system_physics,
    render::{system_render, BlackHoleRenderer},
    sprite_render::system_sprite_render,
};
use crate::systems::keyboard::system_keyboard;
use crate::systems::logic::system_logic;
use crate::systems::orientation::system_orientation;

const FIXED_DT: f32 = 1.0 / 60.0;
const MAX_STEPS: u32 = 8; // spiral-of-death guard

pub fn system_scene(world: &mut World, assets: &AssetManager, sound: &Sound, dt: f32, bh_renderer: &mut Option<BlackHoleRenderer>) {
    // Consume any pending scene transition.
    let pending = {
        let q = world.query_mut::<&mut SceneManager>();
        q.into_iter().next().and_then(|(_, mgr)| mgr.next.take())
    };

    if let Some(kind) = pending {
        clear_scene(world);
        spawn_scene(world, &kind);
        if let Some((_, mgr)) = world.query_mut::<&mut SceneManager>().into_iter().next() {
            mgr.current = kind;
        }
        return; // skip rendering this transition frame
    }

    // Read current scene (clone so we drop the borrow before dispatching).
    let current = {
        let mut q = world.query::<&SceneManager>();
        let mut iter = q.iter();
        iter.next().map(|(_, m)| m.current.clone())
    };

    match current {
        Some(SceneKind::StartMenu) => {
            system_menu_input(world);
            system_menu_render(world);
        }
        Some(SceneKind::Level(n)) => {
            run_level(world, assets, sound, dt, n, bh_renderer);
        }
        _ => {}
    }
}

fn run_level(world: &mut World, assets: &AssetManager, sound: &Sound, dt: f32, level: u32, bh_renderer: &mut Option<BlackHoleRenderer>) {

    system_keyboard(world);
    system_blackhole(world, dt);

    let mut acc = {
        let q = world.query_mut::<&mut PhysicsClock>();
        if let Some((_, clock)) = q.into_iter().next() {
            clock.accumulator += dt;
            clock.global += dt;
            clock.accumulator
        } else {
            dt
        }
    };

    // Fixed-step loop: each tick is exactly FIXED_DT seconds.
    let mut steps = 0;
    while acc >= FIXED_DT && steps < MAX_STEPS {
        system_physics(world, FIXED_DT);
        system_collide(world);

        acc -= FIXED_DT;
        steps += 1;
    }

    if let Some((_, clock)) = world.query_mut::<&mut PhysicsClock>().into_iter().next() {
        clock.accumulator = acc;
    }

    system_logic(world);
    system_audio(world, sound);
    system_orientation(world);
    system_render(world, bh_renderer);
    system_sprite_render(world, assets);
    system_debug(world);

    // HUD
    let fps = get_fps();
    draw_text(&format!("FPS: {fps}"), 10.0, 20.0, 20.0, GRAY);
    let name = if level == 1 { "Level 1 \u{2013} Classic" } else { "Level 2 \u{2013} Gravity Storm" };
    draw_text(name, 10.0, 44.0, 20.0, WHITE);
    draw_text("ESC \u{2013} back to menu", 10.0, 68.0, 20.0, GRAY);

    if is_key_pressed(KeyCode::Escape) {
        if let Some((_, mgr)) = world.query_mut::<&mut SceneManager>().into_iter().next() {
            mgr.next = Some(SceneKind::StartMenu);
        }
    }
}
