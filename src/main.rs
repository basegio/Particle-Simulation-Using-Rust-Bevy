mod particles;
mod simulation;

use bevy::prelude::*;
use particles::ParticlePlugin;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
