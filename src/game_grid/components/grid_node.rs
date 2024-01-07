use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct GridNode {
    pub id: Uuid,
}

impl GridNode {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}
