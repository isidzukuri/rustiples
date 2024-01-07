use bevy::prelude::*;
use uuid::Uuid;

use crate::game_grid::systems::position_allocator::PositionAllocator;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum GridEntityType {
    Castle,
    // Standard,
    // Blocked,
    // RouteHead,
    // RoutePoint,
    // Tree,
    // Axe,
    // Hero,
    // Mineral,
    // Mountain,
}

#[derive(Debug, Clone)]
pub struct GridEntityConfig {
    pub sprite: String,
    pub width_px: f32,
    pub height_px: f32,
    pub margin: (u32, u32, u32, u32),
    pub entity_type: GridEntityType,
}

#[derive(Component, Debug)]
pub struct GridEntity {
    pub id: Uuid,
    pub entity_type: GridEntityType,
    pub x_px: f32,
    pub y_px: f32,
    pub config: GridEntityConfig,
}

#[derive(Component, Debug)]
pub struct GridNode {
    pub id: Uuid,
}

impl GridNode {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

pub struct Entry {
    pub x: u32,
    pub y: u32,
    pub node_id: Uuid,
    pub position_id: Option<Uuid>,
    pub entity_id: Option<Uuid>,
}

impl Entry {
    pub fn new(x: u32, y: u32, node_id: Uuid) -> Self {
        Self {
            x: x,
            y: y,
            node_id: node_id,
            position_id: None,
            entity_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridPosition {
    pub id: Uuid,
    pub width: u32,
    pub height: u32,
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
    pub margin: (u32, u32, u32, u32), // from 12 clockwise
}

#[derive(Resource)]
pub struct Grid {
    pub entries: Vec<Entry>,
    positions: Vec<GridPosition>,
    allocator: PositionAllocator,
    node_size: f32,
}

impl Grid {
    pub fn new(width: u32, height: u32, node_size: f32) -> (Self, Vec<GridNode>) {
        let mut grid = Self {
            entries: vec![],
            positions: vec![],
            allocator: PositionAllocator {
                width: width,
                height: height,
                reserved_cells: vec![],
            },
            node_size: node_size,
        };
        let mut nodes = vec![];

        let mut col_index = 0u32;
        let mut row_index = 0u32;
        loop {
            if row_index == height && col_index == 0 {
                break;
            }

            let node = GridNode::new();
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

    pub fn create_entity(&mut self, config: &GridEntityConfig) -> GridEntity {
        let width = (config.width_px / self.node_size).ceil() as u32;
        let height = (config.height_px / self.node_size).ceil() as u32;
        let width_with_box = width + config.margin.1 + config.margin.3;
        let height_with_box = height + config.margin.0 + config.margin.2;

        match self.allocator.allocate(width_with_box, height_with_box) {
            None => {
                panic!("not possible to allocate space in world for the object")
            }
            Some(allocation) => {
                let grid_position = GridPosition {
                    id: Uuid::new_v4(),
                    width: width,
                    height: height,
                    x1: allocation.x1 + config.margin.3,
                    y1: allocation.y1 + config.margin.2,
                    x2: allocation.x2 - config.margin.1,
                    y2: allocation.y2 - config.margin.0,
                    margin: config.margin.clone(),
                };

                let grid_entity = GridEntity {
                    id: Uuid::new_v4(),
                    entity_type: config.entity_type,
                    x_px: (grid_position.x1 as f32 * self.node_size + config.width_px / 2.0) as f32,
                    y_px: (grid_position.y1 as f32 * self.node_size + config.width_px / 2.0) as f32,
                    config: config.clone(),
                };

                for cur_x in allocation.x1..allocation.x2 {
                    for cur_y in allocation.y1..allocation.y2 {
                        let mut entry = self
                            .entries
                            .iter_mut()
                            .find(|entry| entry.x == cur_x && entry.y == cur_y)
                            .unwrap();
                        entry.position_id = Some(grid_position.id);
                        entry.entity_id = Some(grid_entity.id);
                    }
                }

                self.positions.push(grid_position);

                return grid_entity;
            }
        }
    }

    pub fn get_coords_by_node_id(&self, node_id: &Uuid) -> (u32, u32) {
        match self.entries.iter().find(|entry| &entry.node_id == node_id) {
            Some(entry) => (entry.x, entry.y),
            _ => panic!("Entry with such node_id does not exists in the grid"),
        }
    }

    // pub fn move_entity(){
    // mutate alocator
    // delete current position
    // alocate new position space
    // create new position
    // update self.entries
    // }

    // pub fn delete_entity(){
    // mutate alocator
    // delete current position
    // update self.entries
    // }
}
