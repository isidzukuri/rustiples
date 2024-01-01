use super::world_position::WorldPosition;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Castle {
    pub world_position: WorldPosition,
}

impl Castle {
    pub const MARGIN: (u32, u32, u32, u32) = (1, 1, 1, 1);
    pub const SPRITE_WIDTH: f32 = 350.0f32;
    pub const SPRITE_HEIGHT: f32 = 250.0f32;
}
