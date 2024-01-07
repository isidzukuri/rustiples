use crate::game_grid::grid::GridEntityType;

#[derive(Debug, Clone)]
pub struct GridEntityConfig {
    pub sprite: String,
    pub width_px: f32,
    pub height_px: f32,
    pub margin: (u32, u32, u32, u32),
    pub entity_type: GridEntityType,
}
