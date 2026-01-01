use bevy::diagnostic::{
    Diagnostic, DiagnosticPath, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    RegisterDiagnostic,
};
use bevy::prelude::*;

pub struct DiagnosticPlugin;

impl DiagnosticPlugin {
    pub const SOLVE_COLLISIONS_TIME: DiagnosticPath =
        DiagnosticPath::const_new("physics/solve_collisions");
    pub const GRID_UPDATE_TIME: DiagnosticPath = DiagnosticPath::const_new("grid/update");
    const MAX_HISTORY_LENGTH: usize = 100;
}

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(LogDiagnosticsPlugin::default())
            .register_diagnostic(
                Diagnostic::new(Self::SOLVE_COLLISIONS_TIME)
                    .with_suffix("µs")
                    .with_max_history_length(Self::MAX_HISTORY_LENGTH),
            )
            .register_diagnostic(
                Diagnostic::new(Self::GRID_UPDATE_TIME)
                    .with_suffix("µs")
                    .with_max_history_length(Self::MAX_HISTORY_LENGTH),
            );
    }
}
