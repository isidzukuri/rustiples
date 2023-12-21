use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod components;
mod systems;

use components::*;
use systems::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_counter)
            .add_systems(Update, (fps_text_update_system, fps_counter_showhide));
    }
}
