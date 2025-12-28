use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub width: f32,
    pub height: f32,
    pub gravity: Vec2,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 600.0,
            gravity: Vec2::new(0.0, -500.0),
        }
    }
}
