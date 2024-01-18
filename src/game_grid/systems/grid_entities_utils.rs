use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::grid::*;
use crate::game_grid::grid_entity_factory::GridEntityFactory;



use super::GRID_NODE_SIZE;

pub fn grid_new(window_query: &Query<&Window, With<PrimaryWindow>>) -> (Grid, Vec<GridNode>) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_NODE_SIZE) as u32;
    let height = (window.height() / GRID_NODE_SIZE) as u32;
    Grid::new(width, height, GRID_NODE_SIZE)
}

pub fn node_center_window_coords(coords: &(u32, u32)) -> (f32, f32) {
    let half_size = GRID_NODE_SIZE / 2.0;
    (
        GRID_NODE_SIZE * coords.0 as f32 + half_size,
        GRID_NODE_SIZE * coords.1 as f32 + half_size,
    )
}

pub fn spawn_grid_nodes_sprites(grid: &Grid, nodes: Vec<GridNode>, commands: &mut Commands) {
    for node in nodes {
        let coords = grid.find_coords_by_node_id(&node.id);
        let (window_x, window_y) = node_center_window_coords(&coords);

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
}

fn colorize_node_by_entity(grid: &Grid, node: &GridNode) -> Color {
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

pub fn place_entity(
    grid: &mut Grid,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_type: GridEntityType,
    coords: (u32, u32),
) {
    let grid_entity = GridEntityFactory::create(grid, entity_type, Some(coords));
    spawn_sprite_bundle(commands, asset_server, grid_entity);
}

pub fn spawn_sprite_bundle(
    commands: &mut Commands,
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
