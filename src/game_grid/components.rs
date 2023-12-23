use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum GraphNodeType {
    Standard,
    Blocked,
    RouteHead,
    RoutePoint,
}

#[derive(Component, Debug)]
pub struct GraphNode {
    pub row: u32,
    pub col: u32,
    pub node_type: GraphNodeType,
}
