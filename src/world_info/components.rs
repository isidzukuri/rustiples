use bevy::prelude::*;

/// Marker to find the container entity so we can show/hide
#[derive(Component)]
pub struct WorldInfoRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct WorldInfoText;

#[derive(Component)]
pub struct WorldInfoItem {
    pub val: String,
}
