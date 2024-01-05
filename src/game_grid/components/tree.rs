use crate::game_grid::WorldPosition;
use crate::game_grid::WorldPositionParams;
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

impl WorldPositionParams for Tree {
    fn world_position_params() -> (f32, f32, (u32, u32, u32, u32)) {
        (Self::SPRITE_WIDTH, Self::SPRITE_HEIGHT, Self::MARGIN)
    }
}
