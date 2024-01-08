use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntityType;

pub struct PathfindingParams<'a> {
    pub start_node: &'a (u32, u32),
    pub end_node: &'a (u32, u32),
    pub grid: &'a Grid,
    pub graph_node_types: Vec<GridEntityType>,
    pub axe_position: &'a (u32, u32),
}
