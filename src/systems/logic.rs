use std::collections::HashSet;
use hecs::{Entity, World};
use macroquad::color::Color;
use crate::components::{CollideTag, GameObject, Geometry, LevelManager, LevelState, Position, Velocity, Weight};
use crate::scene::{SceneKind, SceneManager, SceneTag};

// Base on collision system result we will update the logic of the game
pub fn system_logic(world: &mut World) {

    let mut consumed: HashSet<Entity> = HashSet::new();
    let mut to_spawn: Vec<(Position, Velocity, Geometry, Weight)> = Vec::new();

    let mut is_end_of_game = false;
    let mut is_next_level = false;

    for (e1, (obj_1, col)) in
        world.query::<(&GameObject, &CollideTag)>().iter()
    {

        // Skip if either entity is already part of a merge this frame.
        if consumed.contains(&e1) {
            continue;
        }

        // Collision found
        if let Some(e2) = col.other {
            let mut query = world.query_one::<&GameObject>(e2).unwrap();
            let (obj_2) = query.get().unwrap();

            //record it to not handle later
            consumed.insert(e1);
            consumed.insert(e2);

            // Main game logic
            match (obj_1, obj_2) {
                // End of game
                (GameObject::Airship, GameObject::Asteroid) | (GameObject::Asteroid, GameObject::Airship) => {
                    is_end_of_game = true;
                },
                // Next level
                (GameObject::Airship, GameObject::Target) | (GameObject::Target, GameObject::Airship) => {
                    is_next_level = true;
                },
                (GameObject::Asteroid, GameObject::Asteroid) => {
                    let mut query_1 = world.query_one::<(&Position, &Geometry, &Weight, &Velocity)>(e1).unwrap();
                    let (pos_1, geo_1, weight_1, vel_1) = query_1.get().unwrap();

                    let mut query_2 = world.query_one::<(&Position, &Geometry, &Weight, &Velocity)>(e2).unwrap();
                    let (pos_2, geo_2, weight_2, vel_2) = query_2.get().unwrap();

                    let (res_position, res_velocity, res_geometry, res_weight) = collapse(pos_1, geo_1, weight_1, vel_1, pos_2, geo_2, weight_2, vel_2);
                    to_spawn.push((res_position, res_geometry, res_velocity, res_weight));
                },
                _ => ()
            }
        }
    }

    if is_end_of_game {
        let q = world.query_mut::<&mut SceneManager>();
        if let Some((_, mgr)) = q.into_iter().next() {
            mgr.next = Some(SceneKind::StartMenu);
        }
    }

    if is_next_level {
        let q = world.query_mut::<&mut LevelManager>();
        if let Some((_, mgr)) = q.into_iter().next() {
            mgr.state = LevelState::Translating;
            /*if let SceneKind::Level(n) = mgr.current {
                mgr.next = Some(SceneKind::Level(n+1));
            }*/
        }
    }

    // spawn new entity
    for (position, velocity, geometry, weight) in to_spawn {
        world.spawn((
            position,
            velocity,
            geometry,
            GameObject::Asteroid,
            weight,
            SceneTag,
        ));
    }
}

pub fn collapse(
    position_1: &Position, geometry_1: &Geometry, weight_1:&Weight, velocity_1: &Velocity,
    position_2: &Position, geometry_2: &Geometry, weight_2:&Weight, velocity_2: &Velocity)
    -> (Position, Geometry, Velocity, Weight) {
    match (geometry_1, geometry_2) {
        (Geometry::Circle(r1), Geometry::Circle(r2)) => {
            // Treat mass as proportional to radius² (area).
            let m1 = r1 * r1;
            let m2 = r2 * r2;
            let total_m = m1 + m2;

            // Center-of-mass position and conserved momentum.
            let x  = (position_1.x * m1 + position_2.x * m2) / total_m;
            let y  = (position_1.y * m1 + position_2.y * m2) / total_m;
            let dx = (velocity_1.dx * m1 + velocity_2.dx * m2) / total_m;
            let dy = (velocity_1.dy * m1 + velocity_2.dy * m2) / total_m;

            // Summed radius, summed weight, blended color.
            let r = r1 + r2;
            let weight = weight_1.weight + weight_2.weight;

            (
                Position { x, y },
                Geometry::Circle(r),
                Velocity { dx, dy },
                Weight { weight },
            )
        }
        _ => panic!("Geometry collapse is not handled"),
    }
}