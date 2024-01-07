// pub mod ai;
// pub mod game_buttons;
// pub mod generators;
// pub mod grid;
// pub mod position;

// pub use crate::game_grid::ai::pathfinding_params::*;
// pub use crate::game_grid::ai::*;
// pub use crate::game_grid::game_buttons::*;
// pub use crate::game_grid::generators::generate_grid;
// pub use crate::game_grid::generators::*;
// pub use crate::game_grid::graph_node::*;
// pub use crate::game_grid::grid::*;
// pub use crate::game_grid::position::world_position::*;
// pub use crate::game_grid::position::*;

pub mod position_allocator;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::grid::*;
use position_allocator::*;

pub const GRID_CELL_WIDTH: f32 = 50.0 as f32;
pub const HALF_GRID_CELL_WIDTH: f32 = 25.0 as f32;

pub fn generate_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_CELL_WIDTH) as u32;
    let height = (window.height() / GRID_CELL_WIDTH) as u32;
    let (mut grid, nodes) = Grid::new(width, height, GRID_CELL_WIDTH);

    let half_size = GRID_CELL_WIDTH / 2.0;
    for node in nodes {
        let coords = grid.get_coords_by_node_id(&node.id);
        let window_x = GRID_CELL_WIDTH * coords.0 as f32 + half_size;
        let window_y = GRID_CELL_WIDTH * coords.1 as f32 + half_size;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(window_x, window_y, -1.)),
                ..default()
            },
            node,
        ));
    }
    place_entities(&mut grid, &mut commands, window_query, asset_server);
    
    commands.insert_resource(grid);

}


pub fn place_entities(
    grid: &mut Grid,
    mut commands: &mut Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>) {
        
        let grid_entity = GridEntityFactory::create(grid, GridEntityType::Castle);
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

pub struct GridEntityFactory {}

impl GridEntityFactory {
    pub fn create(grid: &mut Grid, obj_type: GridEntityType) -> GridEntity {
        match obj_type {
            GridEntityType::Castle => {
                let config = GridEntityConfig {
                    sprite: "sprites/castle.png".to_string(),
                    width_px: 350.0,
                    height_px: 250.0,
                    margin: (1, 1, 1, 1),
                    entity_type: GridEntityType::Castle
                };

                grid.create_entity(&config)
            },
            _ => panic!("Not registered GridEntityType")
        }        
    }
}




// pub fn spawn_sprite<T>(
//     mut commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     object: T,
//     world_position: WorldPosition,
//     asset_path: &'static str,
// ) where
//     (bevy::prelude::SpriteBundle, T): Bundle,
// {
//     let x = (world_position.from_x_cell as f32 * GRID_CELL_WIDTH + world_position.width_px / 2.0)
//         as f32;
//     let y = (world_position.from_y_cell as f32 * GRID_CELL_WIDTH + world_position.height_px / 2.0)
//         as f32;
//     let transform = Transform::from_xyz(x, y, 0.0);
//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load(asset_path),
//             ..default()
//         },
//         object,
//     ));
// }


// pub fn place_entities(mut grid: ResMut<Grid>,
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>) {
// }