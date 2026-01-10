use bevy::prelude::*;

use crate::{
    particles::components::Particle,
    simulation::resources::{GravityMode, SimulationSettings},
};

pub fn apply_physics(
    time: Res<Time<Fixed>>,
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut particle) in &mut query {
        let acceleration = match settings.gravity {
            GravityMode::Constant(g) => g,
            GravityMode::Point(center, strenght) => {
                let dir = center - particle.position;
                let dist_sq = dir.length_squared().max(8.0);
                dir * strenght / dist_sq
            }
            GravityMode::None => Vec2::ZERO,
        };

        let velocity = particle.position - particle.position_old;
        particle.position_old = particle.position;
        particle.position += velocity + acceleration * (dt * dt);
        transform.translation = particle.position.extend(0.0);
    }
}
