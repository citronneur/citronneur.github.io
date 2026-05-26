use hecs::Entity;
use macroquad::math::Mat2;
use macroquad::prelude::Color;

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone)]
pub struct Weight {
    pub weight: f32
}

#[derive(Clone, PartialEq)]
pub enum GameObject {
    Asteroid,
    Airship,
    Target
}

#[derive(Clone)]
pub enum Geometry {
    Circle(f32),
}

#[derive(Clone)]
pub struct BlackHole {
    pub charging: bool,
}

pub struct CollideTag {
    pub other: Option<Entity>
}

/// Accumulates real time; drives the fixed physics timestep.
pub struct PhysicsClock {
    pub global: f32,
    pub accumulator: f32,
    pub steps: u32
}

pub struct Label(pub String);

pub struct Sprite {
    pub sheet_name: String,
    pub scale: f32,
}

// Marker: ball touched a wall this frame. Written by system_movement, consumed by system_audio.
pub struct BounceTag;

pub struct DebugTag {
    pub print_geometry: bool,
}

pub struct Transformation {
    pub default: Mat2,
    pub transformation: Mat2
}

impl Default for Transformation {
    fn default() -> Self {
        Self { default: Mat2::IDENTITY, transformation: Mat2::IDENTITY }
    }
}

#[derive(Clone)]
pub enum LevelState {
    Running,
    Translating,
    Spawn(u32)
}

#[derive(Clone)]
pub struct LevelManager {
    pub state : LevelState,
    pub density: f32
}

pub struct SpacecraftTag;
