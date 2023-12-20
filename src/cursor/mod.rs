use bevy::prelude::*;

mod systems;
mod components;

use systems::*;
use components::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<Cursor>()
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, move_cursor);
    }
}