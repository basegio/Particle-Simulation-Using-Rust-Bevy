use bevy::prelude::*;

use crate::{
    particles::components::Particle,
    simulation::resources::{GravityMode, SimulationSettings},
};

pub fn apply_physics_logic(dt: &f32, settings: &SimulationSettings, particles: &mut Vec<Particle>) {
    for particle in particles {
        let acceleration = match settings.gravity {
            GravityMode::Constant(g) => g,
            GravityMode::Point(center, strenght) => {
                let dir = center - particle.position;
                let dist_sq = dir.length_squared();
                dir.normalize_or_zero() * strenght / (dist_sq + settings.softening)
            }
            GravityMode::None => Vec2::ZERO,
        };

        let velocity = (particle.position - particle.position_old) * settings.damping;
        particle.position_old = particle.position;
        particle.position += velocity + acceleration * dt;
    }
}
