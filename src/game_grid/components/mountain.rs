use crate::game_grid::WorldPosition;
use crate::game_grid::WorldPositionParams;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Mountain {
    pub world_position: WorldPosition,
}

impl Mountain {
    pub const MARGIN: (u32, u32, u32, u32) = (0, 0, 0, 0);
    pub const SPRITE_WIDTH: f32 = 50.0f32;
    pub const SPRITE_HEIGHT: f32 = 50.0f32;
    pub const SPRITE: &str = "sprites/mountain_50.png";
}

impl WorldPositionParams for Mountain {
    fn world_position_params() -> (f32, f32, (u32, u32, u32, u32)) {
        (Self::SPRITE_WIDTH, Self::SPRITE_HEIGHT, Self::MARGIN)
    }
}
