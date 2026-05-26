use hecs::World;
use macroquad::audio::Sound;
use macroquad::prelude::*;

use crate::assets::AssetManager;
use crate::scene::{clear_scene, spawn_scene, SceneKind, SceneManager};
use crate::systems::{
    menu_input::system_menu_input,
    menu_render::system_menu_render,
    render::{BlackHoleRenderer},
};
use crate::systems::level::system_level;

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
        Some(SceneKind::Level) => {
            system_level(world, assets, sound, dt, bh_renderer);
        }
        _ => {}
    }
}
