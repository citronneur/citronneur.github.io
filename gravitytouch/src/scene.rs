use std::f32::consts::PI;
use hecs::World;
use macroquad::prelude::*;

use crate::components::{CollideTag, DebugTag, GameObject, Geometry, PhysicsClock, Position, Sprite, Transformation, Velocity, Weight};

#[derive(Clone)]
pub enum SceneKind {
    StartMenu,
    Level(u32),
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
        SceneKind::Level(n) => spawn_level(world, *n),
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

fn spawn_level(world: &mut World, level: u32) {
    let sh = screen_height();
    let balls: &[(f32, f32, f32, f32, f32, Color)] = match level {
        1 => &[
            (200.0, 200.0,  150.0,   80.0, 3.0, RED),
            (400.0, 300.0, -120.0,  100.0, 2.0, GREEN),
            //(600.0, 150.0,   90.0, -130.0, 1.0, BLUE),
            //(300.0, 400.0,  -70.0,  -90.0, 2.0, YELLOW),
            //(500.0, 250.0,  110.0,   60.0, 2.0, PURPLE),
        ],
        2 => &[
            (150.0,  80.0,  220.0,   50.0, 2.0, RED),
            (350.0, 120.0, -180.0,   60.0, 1.0, ORANGE),
            (550.0,  80.0,  140.0,  -80.0, 1.0, YELLOW),
            (250.0, 150.0, -200.0,   40.0, 1.0, GREEN),
            (450.0,  90.0,  160.0,   90.0, 2.0, BLUE),
            (650.0, 130.0, -150.0,  -70.0, 1.0, PURPLE),
            (100.0, 200.0,  250.0,   30.0, 1.0, PINK),
            (700.0, 180.0, -220.0,   50.0, 1.0, WHITE),
        ],
        _ => &[],
    };

    // Singleton: drives the fixed physics timestep for this level.
    world.spawn((PhysicsClock { global: 0.0, accumulator: 0.0 }, SceneTag));

    for &(x, y, _dx, _dy, radius, color) in balls {
        world.spawn((
            Position { x  : x / sh, y : y / sh},
            Velocity { dx: 0.0, dy: 0.0 },
            GameObject::Asteroid,
            Geometry::Circle(radius / sh),
            Weight { weight: 1.0 },
            Sprite { sheet_name: "asteroid".to_string(), scale: 0.00006 },
            Transformation {default: Mat2::from_angle(PI/2.0f32), transformation: Mat2::IDENTITY},
            CollideTag { other: None },
            SceneTag,
        ));
    }

    //spawn Airship
    world.spawn((
        Position { x: 0.01, y: 0.01},
        Velocity { dx: 0.01, dy: 0.01 },
        GameObject::Airship,
        Geometry::Circle(1.0 / sh),
        Weight { weight: 1.0 },
        Sprite { sheet_name: "spaceship".to_string(), scale: 0.00012 },
        CollideTag{ other: None },
        Transformation {default: Mat2::from_angle(PI/2.0f32), transformation: Mat2::IDENTITY},
        SceneTag,
    ));

    //spawn Target
    world.spawn((
        Position { x: 1.3, y: 0.9},
        GameObject::Target,
        Geometry::Circle(100.0 / sh),
        Sprite { sheet_name: "player".to_string(), scale: 0.5 },
        Weight { weight: 0.0 },
        CollideTag{ other: None },
        SceneTag,
    ));

    // debug
    world.spawn((DebugTag { print_geometry: false },));
}
