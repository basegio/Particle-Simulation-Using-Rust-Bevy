use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub position_old: Vec2,
    pub radius: f32,
}

impl Particle {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Particle {
            position: position,
            position_old: position,
            radius: radius,
        }
    }
}
