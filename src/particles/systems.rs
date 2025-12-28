use crate::particles::components::Particle;

use bevy::prelude::*;

pub fn spawn_particles(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    for i in 0..5 {
        commands.spawn((
            Particle,
            Transform::from_xyz(i as f32 * 50., 0., 0.),
            GlobalTransform::default(),
        ));
    }
}

pub fn draw_particles(mut gizmos: Gizmos, query: Query<&Transform, With<Particle>>) {
    for transform in &query {
        gizmos.circle_2d(transform.translation.xy(), 10., Color::WHITE);
    }
}
