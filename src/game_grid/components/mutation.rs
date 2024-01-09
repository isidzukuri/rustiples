use crate::game_grid::grid::GridEntityType;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub enum MutationType {
    Create,
    Destroy,
    Move,
}

#[derive(Debug)]
pub struct Mutation {
    pub entity_id: Option<Uuid>,
    pub mutation_type: MutationType,
    pub coords: (u32, u32),
    pub entity_type: Option<GridEntityType>,
}
