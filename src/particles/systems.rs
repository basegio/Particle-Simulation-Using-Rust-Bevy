use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;
use bevy::prelude::*;

pub fn spawn_particles(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            Particle {
                velocity: Vec2::new(i as f32 * 10.0, 0.0),
            },
            Transform::from_xyz(i as f32 * 20.0 - 100.0, 200.0, 0.0),
        ));
    }
}

pub fn apply_physics(
    time: Res<Time<Fixed>>,
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut particle) in &mut query {
        particle.velocity += settings.gravity * dt;
        transform.translation += particle.velocity.extend(0.0) * dt;
    }
}

pub fn draw_particles(mut gizmos: Gizmos, query: Query<&Transform, With<Particle>>) {
    for transform in &query {
        gizmos.circle_2d(transform.translation.xy(), 10.0, Color::WHITE);
    }
}
