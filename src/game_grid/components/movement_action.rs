use super::mutation::Mutation;
use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MovementAction {
    pub path: Vec<(u32, u32)>,
    pub mutations: Vec<Mutation>,
    pub current_step: usize,
    pub grid_entity_id: Uuid,
}

impl MovementAction {
    pub fn new(grid_entity_id: Uuid, path: Vec<(u32, u32)>, mutations: Vec<Mutation>) -> Self {
        Self {
            path: path,
            mutations: mutations,
            current_step: 0,
            grid_entity_id: grid_entity_id,
        }
    }

    pub fn next_coords(&mut self) -> Option<(u32, u32)> {
        let coords = self.path.get(self.current_step);
        self.current_step += 1;
        coords.copied()
    }
}
