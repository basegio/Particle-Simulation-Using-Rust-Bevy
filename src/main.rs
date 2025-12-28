mod particles;

use bevy::prelude::*;

use crate::particles::ParticlePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulador de Partículas do DNS".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ParticlePlugin)
        .run();
}
