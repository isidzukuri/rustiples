
use bevy::prelude::*;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct FpsText;