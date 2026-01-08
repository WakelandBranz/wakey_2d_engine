//! Utilities and common components for working with the Bevy ECS world

use bevy_ecs::component::Component;

use crate::core::colors::colors::{BLACK, BLUE, GREEN, RED, WHITE};

/// A 2D position component
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A 2D velocity component for movement
#[derive(Component, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// A size/dimensions component
#[derive(Component, Clone, Copy, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/// A renderable component with color
/// This allows the engine to draw a shape as a 'sprite'
#[derive(Component, Clone, Copy, Debug)]
pub struct Renderable {
    pub color: [f32; 4],
}

impl Renderable {
    pub fn new(color: [f32; 4]) -> Self {
        Self { color }
    }

    pub fn white() -> Self {
        Self { color: WHITE }
    }

    pub fn red() -> Self {
        Self { color: RED }
    }

    pub fn green() -> Self {
        Self { color: GREEN }
    }

    pub fn blue() -> Self {
        Self { color: BLUE }
    }

    pub fn black() -> Self {
        Self { color: BLACK }
    }
}

/// A collision/bounds component
#[derive(Component, Clone, Copy, Debug)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Bounds {
    pub fn from_position_and_size(pos: Position, size: Size) -> Self {
        Self {
            min_x: pos.x,
            min_y: pos.y,
            max_x: pos.x + size.width,
            max_y: pos.y + size.height,
        }
    }

    pub fn intersects(&self, other: &Bounds) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }
}
