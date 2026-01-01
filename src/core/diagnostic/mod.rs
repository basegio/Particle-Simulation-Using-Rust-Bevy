use bevy::app::Plugin;
use resource::CollisionDiagnostic;

pub mod resource;
pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(CollisionDiagnostic::new(1650));
    }
}
