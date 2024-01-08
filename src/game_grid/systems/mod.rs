pub mod ai;
pub mod grid_click;
pub mod grid_entity_factory;
pub mod grid_generator;
pub mod position_allocator;

pub use crate::game_grid::grid_click::grid_click;
pub use crate::game_grid::grid_generator::generate_grid;

use crate::game_grid::grid::*;

pub const GRID_NODE_SIZE: f32 = 50.0 as f32;
