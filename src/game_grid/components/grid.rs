use bevy::prelude::*;
use uuid::Uuid;

pub use crate::game_grid::entry::*;
pub use crate::game_grid::grid_entity::*;
pub use crate::game_grid::grid_entity_config::*;
pub use crate::game_grid::grid_entity_type::GridEntityType;
pub use crate::game_grid::grid_node::*;
pub use crate::game_grid::grid_position::*;
use crate::game_grid::systems::position_allocator::PositionAllocator;

#[derive(Resource)]
pub struct Grid {
    pub index: Vec<Entry>,
    positions: Vec<GridPosition>,
    allocator: PositionAllocator,
    node_size: f32,
}

impl Grid {
    pub fn new(width: u32, height: u32, node_size: f32) -> (Self, Vec<GridNode>) {
        let mut grid = Self {
            index: vec![],
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

            let node = GridNode::new(col_index, row_index);
            grid.index.push(Entry::new(col_index, row_index, node.id));
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
                    margin: config.margin.clone(),
                    x1: allocation.x1 + config.margin.3,
                    y1: allocation.y1 + config.margin.2,
                    x2: allocation.x2 - config.margin.1,
                    y2: allocation.y2 - config.margin.0,
                };

                let grid_entity = GridEntity {
                    id: Uuid::new_v4(),
                    entity_type: config.entity_type,
                    x_px: (grid_position.x1 as f32 * self.node_size + config.width_px / 2.0) as f32,
                    y_px: (grid_position.y1 as f32 * self.node_size + config.height_px / 2.0)
                        as f32,
                    config: config.clone(),
                };

                for cur_x in grid_position.x1..grid_position.x2 {
                    for cur_y in grid_position.y1..grid_position.y2 {
                        let mut entry = self
                            .index
                            .iter_mut()
                            .find(|entry| entry.x == cur_x && entry.y == cur_y)
                            .unwrap();
                        entry.position_id = Some(grid_position.id);
                        entry.entity_id = Some(grid_entity.id);
                        entry.entity_type = Some(config.entity_type);
                    }
                }

                self.positions.push(grid_position);

                return grid_entity;
            }
        }
    }

    pub fn get_coords_by_node_id(&self, node_id: &Uuid) -> (u32, u32) {
        match self.index.iter().find(|entry| &entry.node_id == node_id) {
            Some(entry) => (entry.x, entry.y),
            _ => panic!("Entry with such node_id does not exists in the grid"),
        }
    }

    pub fn find_entity_type_by_node(&self, node: &GridNode) -> Option<GridEntityType> {
        match self.index.iter().find(|entry| entry.node_id == node.id) {
            Some(entry) => entry.entity_type,
            _ => panic!("Entry with such node_id does not exists in the grid"),
        }
    }

    pub fn find_entry_by_node(&self, node: &GridNode) -> &Entry {
        match self.index.iter().find(|entry| entry.node_id == node.id) {
            Some(entry) => entry,
            _ => panic!("Entry with such node_id does not exists in the grid"),
        }
    }

    pub fn find_entry_by_coords(&self, x: &u32, y: &u32) -> Option<&Entry> {
        self.index
            .iter()
            .find(|entry| &entry.x == x && &entry.y == y)
    }

    pub fn find_coords_by_type(&self, entity_type: GridEntityType) -> Vec<(u32, u32)> {
        let comparable = Some(entity_type);
        let mut result = vec![];
        for entry in self.index.iter() {
            if entry.entity_type == comparable {
                result.push((entry.x, entry.y));
            }
        }
        result
    }

    // pub fn move_entity(){
    // mutate alocator
    // delete current position
    // alocate new position space
    // create new position
    // update self.index
    // }

    // pub fn delete_entity(){
    // mutate alocator
    // delete current position
    // update self.index
    // }
}
