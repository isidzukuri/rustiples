use crate::game_grid::grid_entity_type::GridEntityType;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRAVERSAL_COST: HashMap<GridEntityType, f32> = {
        HashMap::from([
            (GridEntityType::Axe, 1.0),
            (GridEntityType::Tree, 5.0),
            (GridEntityType::Water, 10.0),
            (GridEntityType::Bridge, 2.0),
        ])
    };
}
