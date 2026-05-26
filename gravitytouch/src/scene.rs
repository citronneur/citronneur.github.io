use std::f32::consts::PI;
use hecs::World;
use macroquad::prelude::*;

use crate::components::{CollideTag, DebugTag, GameObject, Geometry, LevelManager, LevelState, PhysicsClock, Position, SpacecraftTag, Sprite, Transformation, Velocity, Weight};

#[derive(Clone)]
pub enum SceneKind {
    StartMenu,
    Level,
    Quit,
}

pub struct SceneManager {
    pub current: SceneKind,
    pub next: Option<SceneKind>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self { current: SceneKind::StartMenu, next: None }
    }
}

/// Marker: entity belongs to the current scene and is cleared on transition.
pub struct SceneTag;

#[derive(Clone)]
pub enum MenuAction {
    StartLevel(u32),
    Quit,
}

pub struct MenuItem {
    pub index: usize,
    pub label: String,
    pub action: MenuAction,
}

/// Marker: this MenuItem is currently highlighted.
pub struct MenuSelected;

pub fn spawn_scene(world: &mut World, kind: &SceneKind) {
    match kind {
        SceneKind::StartMenu => spawn_menu(world),
        SceneKind::Level => spawn_level(world),
        SceneKind::Quit => {}
    }
}

pub fn clear_scene(world: &mut World) {
    let to_remove: Vec<_> = world.query::<&SceneTag>().iter().map(|(e, _)| e).collect();
    for entity in to_remove {
        world.despawn(entity).ok();
    }
}

fn spawn_menu(world: &mut World) {
    let items = [
        ("Level 1 \u{2013} Classic", MenuAction::StartLevel(1)),
        ("Level 2 \u{2013} Gravity Storm", MenuAction::StartLevel(2)),
        ("Quit", MenuAction::Quit),
    ];
    for (i, (label, action)) in items.into_iter().enumerate() {
        let entity = world.spawn((
            MenuItem { index: i, label: label.to_string(), action },
            SceneTag,
        ));
        if i == 0 {
            world.insert(entity, (MenuSelected,)).ok();
        }
    }
}

fn spawn_level(world: &mut World) {
    let sw = screen_width();
    let sh = screen_height();
    let offset = ((sw - 1024.0)/2.0, (sh - 460.0) / 2.0);
    let balls: &[(f32, f32, f32, f32, f32, Color)] = &[
        (200.0, 200.0,  150.0,   80.0, 3.0, RED),
        (400.0, 300.0, -120.0,  100.0, 2.0, GREEN),
        //(600.0, 150.0,   90.0, -130.0, 1.0, BLUE),
        //(300.0, 400.0,  -70.0,  -90.0, 2.0, YELLOW),
        //(500.0, 250.0,  110.0,   60.0, 2.0, PURPLE),
    ];


    // Singleton: drives the fixed physics timestep for this level.
    world.spawn((PhysicsClock { global: 0.0, accumulator: 0.0, steps : 0}, SceneTag));

    for &(x, y, _dx, _dy, radius, color) in balls {
        world.spawn((
            Position { x : x + offset.0, y : y + offset.1},
            Velocity { dx: 0.0, dy: 0.0 },
            GameObject::Asteroid,
            Geometry::Circle(radius),
            Weight { weight: 2.0 },
            Sprite { sheet_name: "asteroid".to_string(), scale: 0.05 },
            Transformation {default: Mat2::from_angle(PI/2.0f32), transformation: Mat2::IDENTITY},
            CollideTag { other: None },
            SceneTag,
        ));
    }

    //spawn spacecraft
   world.spawn((
        Position { x: 10.0 + offset.0 , y: 230.0 + offset.1},
        Velocity { dx: 140.0, dy: 0.0 },
        GameObject::Airship,
        Geometry::Circle(1.0),
        Weight { weight: 1.0 },
        Sprite { sheet_name: "spaceship".to_string(), scale: 0.07 },
        CollideTag{ other: None },
        Transformation {default: Mat2::from_angle(PI/2.0f32), transformation: Mat2::IDENTITY},
        SceneTag,
        SpacecraftTag
    ));

    //spawn Target
    world.spawn((
        Position { x: 990.0 + offset.0, y: 230.0 + offset.1},
        GameObject::Target,
        Geometry::Circle(100.0),
        Sprite { sheet_name: "player".to_string(), scale: 0.5 },
        Weight { weight: 0.0 },
        CollideTag{ other: None },
        SceneTag
    ));

    // debug
    world.spawn((DebugTag { print_geometry: false },));

    world.spawn((LevelManager {
        state : LevelState::Running,
        density: 50.0
    },));
}
