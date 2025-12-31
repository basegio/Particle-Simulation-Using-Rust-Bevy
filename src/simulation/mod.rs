pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::SimulationSettings;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, systems::draw_constraints);
    }
}

fn setup_camera(mut commands: Commands, settings: Res<SimulationSettings>) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: settings.size,
                height: settings.size,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
