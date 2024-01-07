use uuid::Uuid;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GridNode{
    pub id: Uuid,
    pub x: u32,
    pub y: u32,
    pub position_id: Option<Uuid>, 
    pub entity_id: Option<Uuid>, 
}

impl GridNode {
    pub fn new(x: u32, y:u32) -> Self{
        Self {
            id: Uuid::new_v4(),
            x: x,
            y: y, 
            position_id: None,
            entity_id: None
        }
    }
}



pub struct Entry{
    pub x: u32,
    pub y: u32,
    pub node_id: Uuid,
    pub position_id: Option<Uuid>, 
    pub entity_id: Option<Uuid>, 
}

impl Entry {
    pub fn new(x: u32, y:u32, node_id: Uuid) -> Self{
        Self {
            x: x,
            y: y, 
            node_id: node_id,
            position_id: None,
            entity_id: None,
        }
    }
}

pub struct GridPosition {}

#[derive(Resource)]
pub struct Grid {
    pub entries: Vec<Entry>,
    pub positions: Vec<GridPosition>
}

impl Grid {
    pub fn new(width: u32, height: u32) -> (Self, Vec<GridNode>) {
        let mut grid = Self { entries: vec![], positions: vec![] };
        let mut nodes = vec![];

        let mut col_index = 0u32;
        let mut row_index = 0u32;
        loop {
            if row_index == height && col_index == 0 {
                break;
            }
            
            let node = GridNode::new(col_index, row_index);
            grid.entries.push(Entry::new(col_index, row_index, node.id));
            nodes.push(node);

            col_index += 1;
            if col_index == width {
                col_index = 0;
                row_index += 1;
            };
        }
        (grid, nodes)
    }
}
