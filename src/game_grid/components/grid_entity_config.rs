use crate::game_grid::grid::GridEntityType;

#[derive(Debug, Clone)]
pub struct GridEntityConfig {
    pub sprite: String,
    pub width_px: f32,
    pub height_px: f32,
    pub margin: (u32, u32, u32, u32),
    pub entity_type: GridEntityType,
}

impl GridEntityConfig {
    pub fn resolve_config(obj_type: GridEntityType) -> GridEntityConfig {
        match obj_type {
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
            GridEntityType::Water => GridEntityConfig {
                sprite: "sprites/water_50.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            GridEntityType::LumberMill => GridEntityConfig {
                sprite: "sprites/lumber_mill_50.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            GridEntityType::Bridge => GridEntityConfig {
                sprite: "sprites/bridge_50.png".to_string(),
                width_px: 50.0,
                height_px: 50.0,
                margin: (0, 0, 0, 0),
                entity_type: obj_type,
            },
            _ => panic!("Not registered GridEntityType"),
        }
    }
}
