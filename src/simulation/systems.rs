use std::time::Instant;

use crate::diagnostic::DiagnosticPlugin;
use crate::grid::resource::CollisionGrid;
use crate::particles;
use crate::particles::components::Particle;

use super::resources::SimulationSettings;

use bevy::diagnostic::{DiagnosticMeasurement, DiagnosticsStore};
use bevy::prelude::*;

pub fn step_physics_simulation(
    time: Res<Time<Fixed>>,
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
    grid: Res<CollisionGrid>,
    mut diag: ResMut<DiagnosticsStore>,
    mut particles: Local<Vec<Particle>>,
) {
    let dt = time.delta_secs();
    let substeps = settings.substeps;
    let sub_dt = dt / substeps as f32;

    particles.clear();
    particles.extend(query.iter().map(|(_, p)| *p));

    for _ in 0..substeps {
        particles::systems::movement::apply_physics_logic(&sub_dt, &settings, &mut particles);

        let start = Instant::now();
        particles::systems::physics::solve_collisions_logic(&grid, &settings, &mut particles);
        finish_solve_collision_diag(start, &mut diag);

        particles::systems::physics::solve_limits_logic(&settings, &mut particles);
    }

    for (i, (mut transform, mut particle)) in query.iter_mut().enumerate() {
        *particle = particles[i];
        transform.translation = particles[i].position.extend(0.0);
    }

    fn finish_solve_collision_diag(start: Instant, diag: &mut DiagnosticsStore) {
        let elapsed = start.elapsed();
        if let Some(diag) = diag.get_mut(&DiagnosticPlugin::SOLVE_COLLISIONS_TIME) {
            diag.add_measurement(DiagnosticMeasurement {
                time: Instant::now(),
                value: elapsed.as_secs_f64() * 1000000.0,
            });
        }
    }
}

pub fn draw_constraints(mut gizmos: Gizmos, settings: Res<SimulationSettings>) {
    let simulation_size = settings.size as f32;
    gizmos.rect_2d(
        Vec2::ZERO,
        Vec2::new(simulation_size, simulation_size) + Vec2::new(1.0, 1.0),
        Color::linear_rgb(1., 0., 0.),
    );
}
