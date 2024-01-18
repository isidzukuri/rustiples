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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let (mut grid, _) = Grid::new(10, 10, 50.);

        let obj = GridEntityFactory::create(&mut grid, GridEntityType::Tree, Some((0u32, 0u32)));
        assert_eq!(obj.entity_type, GridEntityType::Tree);

        let obj = GridEntityFactory::create(&mut grid, GridEntityType::Axe, None);
        assert_eq!(obj.entity_type, GridEntityType::Axe);
    }

    #[test]
    #[should_panic(expected = "not possible to allocate space in world for the object")]
    fn test_create_failure() {
        let (mut grid, _) = Grid::new(10, 10, 50.);

        let obj = GridEntityFactory::create(&mut grid, GridEntityType::Castle, None);
        let obj = GridEntityFactory::create(&mut grid, GridEntityType::Castle, None);
    }
}
