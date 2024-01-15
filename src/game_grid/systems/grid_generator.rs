use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::grid_entities_utils::*;
use crate::game_grid::grid::*;
use crate::game_grid::grid_entity_factory::GridEntityFactory;

use super::GRID_NODE_SIZE;

pub fn generate_grid(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_NODE_SIZE) as u32;
    let height = (window.height() / GRID_NODE_SIZE) as u32;
    let (mut grid, nodes) = Grid::new(width, height, GRID_NODE_SIZE);

    place_entities_precisely(&mut grid, commands, asset_server);
    place_entities_randomly(&mut grid, commands, asset_server);
    spawn_grid_nodes_sprites(&grid, nodes, commands);

    commands.insert_resource(grid);
}

pub fn place_entities_randomly(
    grid: &mut Grid,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let objcts_to_create = vec![
        (GridEntityType::Castle, 2),
        (GridEntityType::Tree, 15),
        (GridEntityType::Mountain, 10),
        (GridEntityType::Water, 5),
    ];

    for (entity_type, quantity) in objcts_to_create {
        for _ in 0..quantity {
            let grid_entity = GridEntityFactory::create(grid, entity_type, None);
            spawn_sprite_bundle(commands, asset_server, grid_entity);
        }
    }
}

pub fn place_entities_precisely(
    grid: &mut Grid,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let objcts_to_create = vec![
        (GridEntityType::Hero, vec![(0, 0)]),
        (GridEntityType::Axe, vec![(2, 5)]),
        (
            GridEntityType::Tree,
            vec![(10, 0), (10, 1), (11, 1), (12, 1), (12, 0)],
        ),
        (
            GridEntityType::Water,
            vec![
                (21, 0),
                (21, 1),
                (21, 2),
                (22, 0),
                (22, 1),
                (22, 2),
                (23, 1),
            ],
        ),
    ];

    for (entity_type, coords_list) in objcts_to_create {
        for coords in coords_list {
            place_entity(grid, commands, asset_server, entity_type, coords);
        }
    }
}
