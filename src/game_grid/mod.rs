use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
use systems::*;

pub struct GameGridPlugin;

impl Plugin for GameGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_grid))
            .add_systems(Update, (grid_click));

        // app.add_systems(Startup, (generate_grid, spawn_control_buttons))
        //     .add_systems(Update, (grid_click, button_pressed_event_listener));
    }
}
