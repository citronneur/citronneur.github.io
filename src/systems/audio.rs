use hecs::{Entity, World};
use macroquad::audio::{play_sound_once, Sound};

use crate::components::BounceTag;

pub fn system_audio(world: &mut World, sound: &Sound) {
    let bounced: Vec<Entity> = world
        .query::<&BounceTag>()
        .iter()
        .map(|(e, _)| e)
        .collect();

    if !bounced.is_empty() {
        play_sound_once(sound);
        for entity in bounced {
            world.remove::<(BounceTag,)>(entity).ok();
        }
    }
}
