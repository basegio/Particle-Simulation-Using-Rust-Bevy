use super::resources::SimulationSettings;

use bevy::prelude::*;

pub fn draw_constraints(mut gizmos: Gizmos, settings: Res<SimulationSettings>) {
    gizmos.rect_2d(
        Vec2::ZERO,
        Vec2::new(settings.width, settings.height),
        Color::linear_rgb(1., 0., 0.),
    );
}
