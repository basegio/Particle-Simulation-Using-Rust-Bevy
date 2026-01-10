use bevy::prelude::*;

use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

pub fn spawn_particles(mut commands: Commands, settings: Res<SimulationSettings>) {
    let simulation_size = settings.size as f32;
    for i in 1..settings.size {
        for j in 1..settings.size {
            if i % 2 != 0 || j % 2 != 0 {
                continue;
            }

            let max_particle_diameter = settings.max_particle_radius * 2.0;
            let half_simulation_size = simulation_size / 2.0;
            let x: f32 = i as f32 * max_particle_diameter - half_simulation_size;
            let y: f32 = j as f32 * max_particle_diameter - half_simulation_size;
            let position: Vec2 = Vec2 { x: x, y: y };

            if position.distance(Vec2::ZERO) < simulation_size * 0.04
                || position.distance(Vec2::ZERO) > simulation_size * 0.5
            {
                continue;
            };
            commands.spawn((
                Particle::new(position, settings.max_particle_radius),
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}
