use std::time::Instant;

use crate::diagnostic::DiagnosticPlugin;
use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

use bevy::diagnostic::{DiagnosticMeasurement, DiagnosticsStore};
use bevy::math::USizeVec2;
use bevy::prelude::*;

pub fn solve_collisions(
    grid: Res<CollisionGrid>,
    mut query: Query<&mut Particle>,
    mut diag: ResMut<DiagnosticsStore>,
) {
    let start = Instant::now();

    for y in 0..grid.size as i32 {
        for x in 0..grid.size as i32 {
            let neighbors = [(0, 0), (1, 0), (0, 1), (1, 1), (-1, 1)];

            for (dx, dy) in neighbors {
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                if let Some(current_idx) = grid.get_cell_index_from_grid_coord(USizeVec2 {
                    x: x as usize,
                    y: y as usize,
                }) {
                    if let Some(neighbor_idx) = grid.get_cell_index_from_grid_coord(USizeVec2 {
                        x: nx as usize,
                        y: ny as usize,
                    }) {
                        let entities_a = &grid.cells[current_idx];
                        let entities_b = &grid.cells[neighbor_idx];
                        resolve_entities_collisions(
                            entities_a,
                            entities_b,
                            &mut query,
                            current_idx == neighbor_idx,
                        );
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed();
    if let Some(diag) = diag.get_mut(&DiagnosticPlugin::SOLVE_COLLISIONS_TIME) {
        diag.add_measurement(DiagnosticMeasurement {
            time: Instant::now(),
            value: elapsed.as_secs_f64() * 1000000.0,
        });
    }
}

pub fn resolve_entities_collisions(
    entities_a: &[Entity],
    entities_b: &[Entity],
    query: &mut Query<&mut Particle>,
    same_cell: bool,
) {
    for (i, &ent_a) in entities_a.iter().enumerate() {
        let start_index = if same_cell { i + 1 } else { 0 };

        for &ent_b in &entities_b[start_index..] {
            if let Ok([mut p_a, mut p_b]) = query.get_many_mut([ent_a, ent_b]) {
                let collision_axis = p_a.position - p_b.position;
                let dist_sq = collision_axis.length_squared();
                let min_dist = p_a.radius + p_b.radius;

                if dist_sq < min_dist * min_dist {
                    let dist = dist_sq.sqrt();
                    let n = collision_axis / dist;
                    let delta = min_dist - dist;
                    let compensation = n * delta * 0.5;
                    p_a.position += compensation;
                    p_b.position -= compensation;
                }
            }
        }
    }
}

pub fn solve_enviroment_constraints_limits(
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let border_distance = settings.size / 2.0;

    for (mut transform, mut particle) in &mut query {
        if particle.position.y < -border_distance + particle.radius {
            particle.position.y = -border_distance + particle.radius
        } else if particle.position.y > border_distance - particle.radius {
            particle.position.y = border_distance - particle.radius
        }

        if particle.position.x < -border_distance + particle.radius {
            particle.position.x = -border_distance + particle.radius
        } else if particle.position.x > border_distance - particle.radius {
            particle.position.x = border_distance - particle.radius
        }

        transform.translation = particle.position.extend(0.0);
    }
}
