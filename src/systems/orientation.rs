use hecs::World;
use macroquad::math::Mat2;

use crate::components::{GameObject, Transformation, Velocity};

pub fn system_orientation(world: &World) {
    for (_, (vel, orient, obj)) in world.query::<(&Velocity, &mut Transformation, &GameObject)>().iter() {
        let speed_sq = vel.dx * vel.dx + vel.dy * vel.dy;
        if speed_sq < 1e-8 {
            continue;
        }
        let angle = vel.dy.atan2(vel.dx);
        match *obj {
            GameObject::Airship => {
                orient.transformation = Mat2::from_angle(angle);
            },
            GameObject::Asteroid => {
                orient.transformation *= Mat2::from_angle(angle / 200.0 + 0.02);
            }
            _ => ()
        }

    }
}