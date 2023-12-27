use bevy::prelude::*;

#[derive(Event)]
pub struct ButtonPressedEvent {
    pub event_type: String,
}
