use bevy::prelude::*;

#[derive(Component)]
pub struct ClickableButton {
    pub label: String,
    pub event_type: String,
}

#[derive(Component, Debug)]
pub struct Menu {}
