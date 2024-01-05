use bevy::prelude::*;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PATHING_COST: HashMap<GraphNodeType, f32> = {
        HashMap::from([
            (GraphNodeType::Axe, 1.0),
            (GraphNodeType::RouteHead, 1.0),
            (GraphNodeType::Standard, 1.0),
            (GraphNodeType::Tree, 4.0),
        ])
    };
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum GraphNodeType {
    Standard,
    Blocked,
    RouteHead,
    RoutePoint,
    Castle,
    Tree,
    Axe,
    Hero,
    Mineral
}

#[derive(Component, Debug)]
pub struct GraphNode {
    pub row: u32,
    pub col: u32,
    pub node_type: GraphNodeType,
}
