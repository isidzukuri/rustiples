pub mod ai;
pub mod controls;
pub mod grid_click;
pub mod grid_entities_utils;
pub mod grid_entity_factory;
pub mod grid_generator;
pub mod grid_loader;
pub mod position_allocator;

pub use crate::game_grid::grid_click::grid_click;
pub use crate::game_grid::grid_generator::generate_grid;
pub use crate::game_grid::grid_loader::load_grid;

use crate::game_grid::grid::*;

pub const GRID_NODE_SIZE: f32 = 50.0 as f32;
