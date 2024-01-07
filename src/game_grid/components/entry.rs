use crate::game_grid::grid::GridEntityType;
use uuid::Uuid;

pub struct Entry {
    pub x: u32,
    pub y: u32,
    pub node_id: Uuid,
    pub position_id: Option<Uuid>,
    pub entity_id: Option<Uuid>,
    pub entity_type: Option<GridEntityType>,
}

impl Entry {
    pub fn new(x: u32, y: u32, node_id: Uuid) -> Self {
        Self {
            x: x,
            y: y,
            node_id: node_id,
            position_id: None,
            entity_id: None,
            entity_type: None,
        }
    }
}
