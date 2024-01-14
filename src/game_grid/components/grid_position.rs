use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
