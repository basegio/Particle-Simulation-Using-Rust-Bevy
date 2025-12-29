use bevy::prelude::*;

use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

pub fn spawn_particles(mut commands: Commands, settings: Res<SimulationSettings>) {
    for i in 1..32 {
        for j in 1..32 {
            if i % 2 != 0 || j % 2 != 0 {
                continue;
            }

            let x: f32 = i as f32 - settings.size / 2.0;
            let y: f32 = j as f32 - settings.size / 2.0;
            let position: Vec2 = Vec2 { x: x, y: y };
            commands.spawn((
                Particle::from_position(position),
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}
