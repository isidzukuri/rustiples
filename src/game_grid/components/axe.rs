use super::world_position::WorldPosition;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Axe {
    pub world_position: WorldPosition,
}

impl Axe {
    pub const MARGIN: (u32, u32, u32, u32) = (0,0, 0, 0);
    pub const SPRITE_WIDTH: f32 = 50.0f32;
    pub const SPRITE_HEIGHT: f32 = 50.0f32;
}
