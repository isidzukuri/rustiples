use bevy::prelude::*;

mod components;
mod systems;

use components::*;
use systems::*;

pub struct GameGridPlugin;

impl Plugin for GameGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_grid)
            .add_systems(Update, grid_click);
    }
}
