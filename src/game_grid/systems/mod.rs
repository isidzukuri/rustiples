pub mod ai;
pub mod grid_click;
pub mod grid_entity_factory;
pub mod position_allocator;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::grid::*;
pub use crate::game_grid::grid_click::grid_click;
use crate::game_grid::grid_entity_factory::GridEntityFactory;

pub const GRID_NODE_SIZE: f32 = 50.0 as f32;

pub fn generate_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_NODE_SIZE) as u32;
    let height = (window.height() / GRID_NODE_SIZE) as u32;
    let (mut grid, nodes) = Grid::new(width, height, GRID_NODE_SIZE);

    place_entities(&mut grid, &mut commands, window_query, asset_server);

    let half_size = GRID_NODE_SIZE / 2.0;
    for node in nodes {
        let coords = grid.get_coords_by_node_id(&node.id);
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

pub fn place_entities(
    grid: &mut Grid,
    mut commands: &mut Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let objcts_to_create = vec![
        (GridEntityType::Castle, 2),
        (GridEntityType::Hero, 1),
        (GridEntityType::Axe, 1),
        (GridEntityType::Tree, 10),
        (GridEntityType::Mountain, 10),
        // (GridEntityType::Water, 5),
    ];

    for (entity_type, quantity) in objcts_to_create {
        for _ in 0..quantity {
            let grid_entity = GridEntityFactory::create(grid, entity_type);
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
    }
}
