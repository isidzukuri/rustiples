use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridEntityConfig;
use crate::game_grid::systems::GridEntityType;

pub struct GridEntityFactory {}

impl GridEntityFactory {
    pub fn create(grid: &mut Grid, obj_type: GridEntityType) -> GridEntity {
        let config = match obj_type {
            GridEntityType::Castle => GridEntityConfig {
                sprite: "sprites/castle.png".to_string(),
                width_px: 350.0,
                height_px: 250.0,
                margin: (1, 1, 1, 1),
                entity_type: obj_type,
            },
            GridEntityType::Hero => GridEntityConfig {
                sprite: "sprites/hero.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            GridEntityType::Axe => GridEntityConfig {
                sprite: "sprites/axe.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            GridEntityType::Tree => GridEntityConfig {
                sprite: "sprites/tree.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            GridEntityType::Mountain => GridEntityConfig {
                sprite: "sprites/mountain_50.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            // GridEntityType::Water => {
            //     GridEntityConfig {
            //        sprite: "sprites/water_50.png".to_string(),
            //        width_px: 50.0,
            //        height_px: 50.0,
            //        margin: (0, 0, 0, 0),
            //        entity_type: obj_type,
            //    }
            // },
            _ => panic!("Not registered GridEntityType"),
        };

        grid.create_entity(&config)
    }
}
