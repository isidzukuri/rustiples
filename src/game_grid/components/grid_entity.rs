use bevy::prelude::*;
use uuid::Uuid;

pub use crate::game_grid::grid_entity_config::*;
pub use crate::game_grid::grid_entity_type::GridEntityType;

#[derive(Component, Debug)]
pub struct GridEntity {
    pub id: Uuid,
    pub entity_type: GridEntityType,
    pub x_px: f32,
    pub y_px: f32,
    pub config: GridEntityConfig,
}
