use super::world_position::WorldPosition;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Tree {
    pub world_position: WorldPosition,
}

impl Tree {
    pub const MARGIN: (u32, u32, u32, u32) = (0, 0, 0, 0);
    pub const SPRITE_WIDTH: f32 = 50.0f32;
    pub const SPRITE_HEIGHT: f32 = 50.0f32;
    pub const SPRITE: &str = "sprites/tree.png";
}
