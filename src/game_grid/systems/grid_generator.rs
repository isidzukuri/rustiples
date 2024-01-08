use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::grid::*;
use crate::game_grid::grid_entity_factory::GridEntityFactory;

use super::GRID_NODE_SIZE;

pub fn generate_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_NODE_SIZE) as u32;
    let height = (window.height() / GRID_NODE_SIZE) as u32;
    let (mut grid, nodes) = Grid::new(width, height, GRID_NODE_SIZE);

    place_entities_precisely(&mut grid, &mut commands, &asset_server);
    place_entities_randomly(&mut grid, &mut commands, &asset_server);

    let half_size = GRID_NODE_SIZE / 2.0;
    for node in nodes {
        let coords = grid.find_coords_by_node_id(&node.id);
        let window_x = GRID_NODE_SIZE * coords.0 as f32 + half_size;
        let window_y = GRID_NODE_SIZE * coords.1 as f32 + half_size;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: colorize_node_by_entity(&grid, &node),
                    custom_size: Some(Vec2::new(GRID_NODE_SIZE, GRID_NODE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(window_x, window_y, -1.)),
                ..default()
            },
            node,
        ));
    }

    commands.insert_resource(grid);
}

pub fn colorize_node_by_entity(grid: &Grid, node: &GridNode) -> Color {
    if let Some(entity_type) = grid.find_entity_type_by_node(node) {
        match entity_type {
            GridEntityType::Castle => Color::GOLD,
            GridEntityType::Hero => Color::ANTIQUE_WHITE,
            GridEntityType::Axe => Color::INDIGO,
            GridEntityType::Tree => Color::LIME_GREEN,
            GridEntityType::Mountain => Color::DARK_GRAY,
            _ => Color::GRAY,
        }
    } else {
        Color::GRAY
    }
}

pub fn place_entities_randomly(
    grid: &mut Grid,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let objcts_to_create = vec![
        (GridEntityType::Castle, 2),
        (GridEntityType::Tree, 10),
        (GridEntityType::Mountain, 10),
        // (GridEntityType::Water, 5),
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
    ];

    for (entity_type, coords_list) in objcts_to_create {
        for coords in coords_list {
            let grid_entity = GridEntityFactory::create(grid, entity_type, Some(coords));
            spawn_sprite_bundle(commands, asset_server, grid_entity);
        }
    }
}

pub fn spawn_sprite_bundle(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_entity: GridEntity,
) {
    let transform = Transform::from_xyz(grid_entity.x_px, grid_entity.y_px, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: transform,
            texture: asset_server.load(grid_entity.config.sprite.clone()),
            ..default()
        },
        grid_entity,
    ));
}
