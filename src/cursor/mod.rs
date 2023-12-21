use bevy::prelude::*;

mod components;
mod systems;

use components::*;
use systems::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(Update, move_cursor);
    }
}
