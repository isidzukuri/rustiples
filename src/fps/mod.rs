use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod systems;
mod components;

use systems::*;
use components::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_counter)
            .add_systems(Update, (
                fps_text_update_system,
                fps_counter_showhide,
            ));
    }
}