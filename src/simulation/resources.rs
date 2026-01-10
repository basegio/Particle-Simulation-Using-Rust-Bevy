use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub size: usize,
    pub max_particle_radius: f32,
    pub gravity: GravityMode,
    pub friction: f32,
    pub collision_friction: f32,
    pub relaxation_factor: f32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            size: 1024,
            max_particle_radius: 2.0,
            // gravity: GravityMode::Constant(Vec2::new(0.0, -9.8)),
            gravity: GravityMode::Point(Vec2::ZERO, 24000.0),
            // gravity: GravityMode::None,
            friction: 0.98,
            collision_friction: 0.1,
            relaxation_factor: 0.5,
        }
    }
}

#[derive(Resource)]
pub enum GravityMode {
    Constant(Vec2),
    Point(Vec2, f32),
    None,
}
