use crate::game_grid::GraphNodeType;
use crate::game_grid::GraphNode;

pub struct PathfindingParams<'a> {
    pub start_node: &'a (u32, u32),
    pub end_node: &'a (u32, u32),
    pub game_grid_nodes: &'a Vec<&'a GraphNode>,
    pub graph_node_types: Vec<GraphNodeType>
}