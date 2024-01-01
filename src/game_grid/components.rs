use bevy::prelude::*;
// use crate::game_grid::components::WorldObject;

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

// pub trait WorldObject {
//     fn is_owned_cell(&self, x: &u32, y: &u32) -> bool;
//     fn intersects_with(&self, other: &T) -> bool;
// }

// macro_rules! implement_world_object {
//     ($($t:ty),+ $(,)?) => ($(
//         impl WorldObject for $t {
//             fn is_owned_cell(&self, x: &u32, y: &u32) -> bool {
//                 if self.from_x_cell <= *x
//                     && self.to_x_cell >= *x
//                     && self.from_y_cell <= *y
//                     && self.to_y_cell >= *y
//                 {
//                     true
//                 } else {
//                     false
//                 }
//             }

//             fn intersects_with(&self, other: &T) -> bool {
//                 if (self.from_x_cell < other.to_x_cell && self.to_x_cell > other.from_x_cell &&
//                     self.from_y_cell > other.to_y_cell && self.to_y_cell < other.from_y_cell)
//                 {
//                     true
//                 } else {
//                     false
//                 }
//             }
//         }
//     )+)
// }

// implement_world_object!(Castle);
#[derive(Component, Debug)]
pub struct WorldPosition {
    pub width_px: f32,
    pub height_px: f32,
    pub width_cells: u32,
    pub height_cells: u32,
    pub from_x_cell: u32,
    pub from_y_cell: u32,
    pub to_x_cell: u32,
    pub to_y_cell: u32,
}

impl WorldPosition {
    pub fn is_owned_cell(&self, x: &u32, y: &u32) -> bool {
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

    pub fn intersects_with(&self, other: &Self) -> bool {
        if (self.from_x_cell < other.to_x_cell
            && self.to_x_cell > other.from_x_cell
            && self.from_y_cell > other.to_y_cell
            && self.to_y_cell < other.from_y_cell)
        {
            true
        } else {
            false
        }
    }
}

#[derive(Component, Debug)]
pub struct Castle {
    pub world_position: WorldPosition,
}
