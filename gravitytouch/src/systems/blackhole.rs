use hecs::World;
use macroquad::prelude::*;

use crate::components::{BlackHole, Position, Weight};
use crate::scene::SceneTag;

const WEIGHT_RATE: f32 = 10.0;  // units of weight per second held
const WEIGHT_MIN:  f32 = 1.0;   // minimum even for a quick tap

pub fn system_blackhole(world: &mut World, dt: f32) {
    let pressing     = is_mouse_button_down(MouseButton::Left);
    let just_pressed = is_mouse_button_pressed(MouseButton::Left);
    let released     = is_mouse_button_released(MouseButton::Left);
    let sh = screen_height();

    // Spawn the entity on the first frame of a press.
    if just_pressed {
        let (mx, my) = mouse_position();

        world.spawn((
            Position { x: mx/sh, y: my/sh},
            Weight { weight: 1.0 },
            BlackHole { charging: true },
            SceneTag,
        ));
    }

    if pressing || released {
        for (_, (weight, hole)) in world.query_mut::<(&mut Weight,  &mut BlackHole)>() {
            if !hole.charging { continue; }

            if pressing {
                weight.weight = (weight.weight + dt * WEIGHT_RATE);
            }
            if released {
                weight.weight = weight.weight.max(WEIGHT_MIN);
                hole.charging = false;
            }
        }
    }
}
