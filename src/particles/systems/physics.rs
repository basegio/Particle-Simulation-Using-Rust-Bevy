use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

use bevy::math::USizeVec2;
use bevy::prelude::*;

pub fn solve_collisions_logic(
    grid: &CollisionGrid,
    settings: &SimulationSettings,
    query: &mut Query<(&mut Transform, &mut Particle)>,
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
                        let entities_a = &grid.cells[current_idx];
                        let entities_b = &grid.cells[neighbor_idx];
                        resolve_entities_collisions(
                            entities_a,
                            entities_b,
                            query,
                            current_idx == neighbor_idx,
                            settings,
                        );
                    }
                }
            }
        }
    }
}

pub fn resolve_entities_collisions(
    entities_a: &[Entity],
    entities_b: &[Entity],
    query: &mut Query<(&mut Transform, &mut Particle)>,
    same_cell: bool,
    settings: &SimulationSettings,
) {
    for (i, &ent_a) in entities_a.iter().enumerate() {
        let start_index = if same_cell { i + 1 } else { 0 };

        for &ent_b in &entities_b[start_index..] {
            if let Ok([(_, mut p_a), (_, mut p_b)]) = query.get_many_mut([ent_a, ent_b]) {
                let collision_axis = p_a.position - p_b.position;
                let dist_sq = collision_axis.length_squared();
                let min_dist = p_a.radius + p_b.radius;

                if dist_sq < min_dist * min_dist {
                    let dist = dist_sq.sqrt();
                    let n = collision_axis / dist;
                    let delta = min_dist - dist;
                    let compensation = n * delta * settings.relaxation_factor;

                    p_a.position += compensation;
                    p_b.position -= compensation;

                    p_a.position_old += compensation * settings.restitution_amortization;
                    p_b.position_old -= compensation * settings.restitution_amortization;
                }
            }
        }
    }
}

pub fn solve_limits_logic(
    settings: &SimulationSettings,
    query: &mut Query<(&mut Transform, &mut Particle)>,
) {
    let border_distance = settings.size as f32 / 2.0;

    for (_, mut particle) in query {
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
