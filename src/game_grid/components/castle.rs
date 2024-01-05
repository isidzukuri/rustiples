use super::world_position::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Castle {
    pub world_position: WorldPosition,
}

impl Castle {
    pub const MARGIN: (u32, u32, u32, u32) = (1, 1, 1, 1);
    pub const SPRITE_WIDTH: f32 = 350.0f32;
    pub const SPRITE_HEIGHT: f32 = 250.0f32;
    pub const SPRITE: &str = "sprites/castle.png";
}

impl WorldPositionParams for Castle {
    fn world_position_params() -> (f32, f32, (u32, u32, u32, u32)){
        (Self::SPRITE_WIDTH, Self::SPRITE_HEIGHT, Self::MARGIN)
    } 
}