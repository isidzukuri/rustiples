use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct GridNode {
    pub id: Uuid,
    pub x: u32,
    pub y: u32,
}

impl GridNode {
    pub fn new(x: u32, y: u32) -> Self {
        Self { id: Uuid::new_v4(), x: x, y: y }
    }
}
