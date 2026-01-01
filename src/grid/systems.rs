use std::time::Instant;

use bevy::diagnostic::{DiagnosticMeasurement, DiagnosticsStore};
use bevy::prelude::*;

use crate::diagnostic::DiagnosticPlugin;
use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;

pub fn update_grid(
    mut grid: ResMut<CollisionGrid>,
    query: Query<(Entity, &Particle)>,
    mut diag: ResMut<DiagnosticsStore>,
) {
    let start = Instant::now();

    grid.clear();
    for (entity, particle) in &query {
        if let Some(idx) = grid.get_cell_index_from_particle_pos(particle.position) {
            grid.cells[idx].push(entity);
        }
    }

    let elapsed = start.elapsed();
    if let Some(diag) = diag.get_mut(&DiagnosticPlugin::GRID_UPDATE_TIME) {
        diag.add_measurement(DiagnosticMeasurement {
            time: Instant::now(),
            value: elapsed.as_secs_f64() * 1000000.0,
        });
    }
}
