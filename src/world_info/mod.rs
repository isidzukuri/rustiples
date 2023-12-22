use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
use systems::*;
pub use systems::print_world_info;

pub struct WorldInfoPlugin;

impl Plugin for WorldInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_world_info, print_window_size))
            .add_systems(Update, (world_info_text_update_system, world_info_showhide));
    }
}
