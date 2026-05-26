use hecs::World;
use macroquad::input::{is_key_pressed, KeyCode};
use crate::components::{DebugTag};
use crate::scene::{SceneKind, SceneManager};

pub fn system_keyboard(world: &mut World) {
    if is_key_pressed(KeyCode::F1) {
        if let Some((_, debug)) = world.query_mut::<&mut DebugTag>().into_iter().next() {
            debug.print_geometry = !debug.print_geometry;
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        if let Some((_, mgr)) = world.query_mut::<&mut SceneManager>().into_iter().next() {
            mgr.next = Some(SceneKind::StartMenu);
        }
    }
}