use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridEntityConfig;
use crate::game_grid::systems::GridEntityType;

pub struct GridEntityFactory {}

impl GridEntityFactory {
    pub fn create(
        grid: &mut Grid,
        obj_type: GridEntityType,
        at_coords: Option<(u32, u32)>,
    ) -> GridEntity {
        let config = GridEntityConfig::resolve_config(obj_type);

        grid.create_entity(&config, at_coords)
    }
}
