use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::fs;

use super::grid_entities_utils::*;
use super::GRID_NODE_SIZE;
use crate::game_grid::grid::*;

pub fn load_grid(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    println!("Game loading...");

    let data = fs::read_to_string("grids/save.json").expect("Unable to read file");

    let temporary_grid: Grid = serde_json::from_str(&data).unwrap();

    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_NODE_SIZE) as u32;
    let height = (window.height() / GRID_NODE_SIZE) as u32;
    let (mut grid, nodes) = Grid::new(width, height, GRID_NODE_SIZE);

    let mut used_positions = vec![];

    for entry in temporary_grid
        .index()
        .iter()
        .filter(|item| item.entity_type.is_some())
    {
        let position_id = entry.position_id.unwrap();
        if used_positions.contains(&position_id) {
            continue;
        };

        let position = temporary_grid.find_position_by_position_id(&position_id);
        let coords = (position.x1, position.y1);
        place_entity(
            &mut grid,
            commands,
            asset_server,
            entry.entity_type.unwrap(),
            coords,
        );
        used_positions.push(position_id);
    }

    spawn_grid_nodes_sprites(&grid, nodes, commands);

    commands.insert_resource(grid);
}
