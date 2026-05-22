mod assets;
mod audio;
mod components;
mod scene;
mod systems;

use assets::AssetManager;
use audio::generate_bounce_wav;
use macroquad::audio::load_sound_from_bytes;
use macroquad::prelude::*;
use scene::{spawn_scene, SceneKind, SceneManager};
use systems::render::BlackHoleRenderer;
use systems::scene::system_scene;

#[macroquad::main("GravityTouch")]
async fn main() {
    let mut assets = AssetManager::new();
    assets.load("spaceship", "assets/spaceship_sprite.png").await.expect("assets/spaceship_sprite.png");
    assets.load("asteroid", "assets/asteroid.png").await.expect("assets/asteroid.png");
    assets.register_sheet("spaceship", "spaceship", 540.0, 915.0, 4);
    assets.register_sheet("asteroid", "asteroid", 660.0, 600.0, 1);

    let bounce_sound = load_sound_from_bytes(&generate_bounce_wav())
        .await
        .expect("bounce sound");

    let mut world = hecs::World::new();
    let mut bh_renderer: Option<BlackHoleRenderer> = None;

    // Singleton entity that drives scene transitions.
    world.spawn((SceneManager::new(),));

    // Spawn initial scene entities.
    spawn_scene(&mut world, &SceneKind::StartMenu);

    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));

        let dt = get_frame_time();
        system_scene(&mut world, &assets, &bounce_sound, dt, &mut bh_renderer);

        // Quit when the scene manager reaches the Quit state.
        let quit = {
            let mut q = world.query::<&SceneManager>();
            let mut iter = q.iter();
            iter.next().map(|(_, m)| matches!(m.current, SceneKind::Quit)).unwrap_or(false)
        };
        if quit {
            break;
        }

        next_frame().await;
    }
}
