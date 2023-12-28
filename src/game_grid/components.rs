use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum GraphNodeType {
    Standard,
    Blocked,
    RouteHead,
    RoutePoint,
    Castle,
}

#[derive(Component, Debug)]
pub struct GraphNode {
    pub row: u32,
    pub col: u32,
    pub node_type: GraphNodeType,
}

#[derive(Component, Debug)]
pub struct Castle {
    pub width_px: f32,
    pub height_px: f32,
    pub width_cells: u32,
    pub height_cells: u32,
    pub from_x_cell: u32,
    pub from_y_cell: u32,
    pub to_x_cell: u32,
    pub to_y_cell: u32,
}

impl Castle {
    pub fn is_catle_cell(&self, x: &u32, y: &u32) -> bool {
        if self.from_x_cell <= *x
            && self.to_x_cell >= *x
            && self.from_y_cell <= *y
            && self.to_y_cell >= *y
        {
            true
        } else {
            false
        }
    }
}
