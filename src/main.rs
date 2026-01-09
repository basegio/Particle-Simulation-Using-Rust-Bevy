mod core;
mod diagnostic;
mod grid;
mod particles;
mod simulation;

use bevy::prelude::*;

use core::CorePlugin;
use diagnostic::DiagnosticPlugin;
use grid::GridPlugin;
use particles::ParticlePlugin;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (720, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_plugins(DiagnosticPlugin)
        .add_plugins(CorePlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
