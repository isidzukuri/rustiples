use bevy::prelude::*;

mod components;
mod events;
mod systems;

pub use components::*;
pub use events::*;
pub use systems::*;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonPressedEvent>()
            .add_systems(Update, event_publisher);
    }
}
