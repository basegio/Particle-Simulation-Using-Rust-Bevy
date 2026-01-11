use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

use bevy::math::USizeVec2;
use bevy::prelude::*;

pub fn solve_collisions_logic(
    grid: &CollisionGrid,
    settings: &SimulationSettings,
    particles: &mut Vec<Particle>,
) {
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
                        let indexes_a = &grid.cells[current_idx];
                        let indexes_b = &grid.cells[neighbor_idx];
                        resolve_indexes_collisions(
                            indexes_a,
                            indexes_b,
                            particles,
                            current_idx == neighbor_idx,
                            settings,
                        );
                    }
                }
            }
        }
    }
}

pub fn resolve_indexes_collisions(
    indexes_a: &[usize],
    indexes_b: &[usize],
    particles: &mut Vec<Particle>,
    same_cell: bool,
    settings: &SimulationSettings,
) {
    for (i, &idx_a) in indexes_a.iter().enumerate() {
        let start_index = if same_cell { i + 1 } else { 0 };

        for &idx_b in &indexes_b[start_index..] {
            let p_a_pos = particles[idx_a].position;
            let p_b_pos = particles[idx_b].position;
            let p_a_rad = particles[idx_a].radius;
            let p_b_rad = particles[idx_b].radius;

            let collision_axis = p_a_pos - p_b_pos;
            let dist_sq = collision_axis.length_squared();
            let min_dist = p_a_rad + p_b_rad;

            if dist_sq < min_dist * min_dist {
                let dist = dist_sq.sqrt();
                let n = collision_axis / dist;
                let delta = min_dist - dist;
                let compensation = n * delta * settings.relaxation_factor;

                particles[idx_a].position += compensation;
                particles[idx_b].position -= compensation;

                particles[idx_a].position_old += compensation * settings.restitution_amortization;
                particles[idx_b].position_old -= compensation * settings.restitution_amortization;
            }
        }
    }
}

pub fn solve_limits_logic(settings: &SimulationSettings, particles: &mut Vec<Particle>) {
    let border_distance = settings.size as f32 / 2.0;

    for particle in particles {
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
    }
}
