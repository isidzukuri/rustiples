use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::game_grid::axe::Axe;
use crate::game_grid::castle::Castle;
use crate::game_grid::graph_node::GraphNodeType;
use crate::game_grid::graph_node::*;
use crate::game_grid::hero::Hero;
use crate::game_grid::mountain::Mountain;
use crate::game_grid::tree::Tree;
use crate::game_grid::world_position::{WorldPosition, WorldPositionParams};
use crate::game_grid::position_alocator::PositionAllocator;

pub const GRID_CELL_WIDTH: f32 = 50.0 as f32;
pub const HALF_GRID_CELL_WIDTH: f32 = 25.0 as f32;

pub fn generate_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let width_in_cells = (window.width() / GRID_CELL_WIDTH) as u32;
    let height_in_cells = (window.height() / GRID_CELL_WIDTH) as u32;

    let mut position_allocator = PositionAllocator {
        width_cells: width_in_cells,
        height_cells: height_in_cells,
        reserved_cells: vec![],
    };

    let heroes_positions =
        allocate_heroes(&width_in_cells, &height_in_cells, &mut position_allocator);
    let axes_positions = allocate_axes(&width_in_cells, &height_in_cells, &mut position_allocator);
    let castle_positions = allocate_positions(
        2,
        &width_in_cells,
        &height_in_cells,
        Castle::world_position_params(),
        &mut position_allocator,
    );
    let trees_positions = allocate_positions(
        20,
        &width_in_cells,
        &height_in_cells,
        Tree::world_position_params(),
        &mut position_allocator,
    );
    let mountains_positions = allocate_positions(
        20,
        &width_in_cells,
        &height_in_cells,
        Mountain::world_position_params(),
        &mut position_allocator,
    );
    // let trees_positions = allocate_trees(&width_in_cells, &height_in_cells);

    let mut col_index = 0u32;
    let mut row_index = 0u32;
    loop {
        if row_index == height_in_cells && col_index == 0 {
            break;
        }

        let x = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * col_index as f32;
        let y = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * row_index as f32;

        let random_num: u16 = rand::thread_rng().gen_range(1..50000);

        let is_castle = castle_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        let is_hero = heroes_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        let is_axe = axes_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        let is_tree = trees_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        let is_mountain = mountains_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if is_castle {
                        // Color::GRAY
                        Color::GOLD
                    } else if is_hero {
                        Color::ANTIQUE_WHITE
                    } else if is_axe {
                        Color::INDIGO
                    } else if is_tree {
                        Color::LIME_GREEN
                    } else if random_num == 1 {
                        Color::ORANGE
                    } else {
                        Color::GRAY
                    },
                    custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, -1.)),
                ..default()
            },
            GraphNode {
                row: row_index,
                col: col_index,
                node_type: if is_castle {
                    GraphNodeType::Castle
                } else if is_hero {
                    GraphNodeType::Hero
                } else if is_axe {
                    GraphNodeType::Axe
                } else if is_tree {
                    GraphNodeType::Tree
                } else if is_mountain {
                    GraphNodeType::Mountain
                } else if random_num == 1 {
                    GraphNodeType::Blocked
                } else {
                    GraphNodeType::Standard
                },
            },
        ));

        col_index += 1;
        if col_index == width_in_cells {
            col_index = 0;
            row_index += 1;
        };
    }

    for position in castle_positions {
        let obj = Castle {
            world_position: position.clone(),
        };
        spawn_sprite(&mut commands, &asset_server, obj, position, Castle::SPRITE)
    }
    for position in heroes_positions {
        let obj = Hero {
            world_position: position.clone(),
            has_axe: true,
        };
        spawn_sprite(&mut commands, &asset_server, obj, position, Hero::SPRITE)
    }
    for position in axes_positions {
        let obj = Axe {
            world_position: position.clone(),
        };
        spawn_sprite(&mut commands, &asset_server, obj, position, Axe::SPRITE)
    }
    for position in trees_positions {
        let obj = Tree {
            world_position: position.clone(),
        };
        spawn_sprite(&mut commands, &asset_server, obj, position, Tree::SPRITE)
    }
    for position in mountains_positions {
        let obj = Tree {
            world_position: position.clone(),
        };
        spawn_sprite(
            &mut commands,
            &asset_server,
            obj,
            position,
            Mountain::SPRITE,
        )
    }
}

pub fn spawn_sprite<T>(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    object: T,
    world_position: WorldPosition,
    asset_path: &'static str,
) where
    (bevy::prelude::SpriteBundle, T): Bundle,
{
    let x = (world_position.from_x_cell as f32 * GRID_CELL_WIDTH + world_position.width_px / 2.0)
        as f32;
    let y = (world_position.from_y_cell as f32 * GRID_CELL_WIDTH + world_position.height_px / 2.0)
        as f32;
    let transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: transform,
            texture: asset_server.load(asset_path),
            ..default()
        },
        object,
    ));
}

pub fn allocate_heroes(
    width_in_cells: &u32,
    height_in_cells: &u32,
    position_allocator: &mut PositionAllocator,
) -> Vec<WorldPosition> {
    vec![WorldPosition::alocate_at(
        &0,
        &0,
        &Hero::SPRITE_WIDTH,
        &Hero::SPRITE_HEIGHT,
        &&GRID_CELL_WIDTH,
        &Hero::MARGIN,
        position_allocator,
    )]
}

pub fn allocate_axes(
    width_in_cells: &u32,
    height_in_cells: &u32,
    position_allocator: &mut PositionAllocator,
) -> Vec<WorldPosition> {
    vec![WorldPosition::alocate_at(
        &1,
        &12,
        &Axe::SPRITE_WIDTH,
        &Axe::SPRITE_HEIGHT,
        &&GRID_CELL_WIDTH,
        &Axe::MARGIN,
        position_allocator,
    )]
}

pub fn allocate_positions(
    quantity: usize,
    width_in_cells: &u32,
    height_in_cells: &u32,
    position_params: (f32, f32, (u32, u32, u32, u32)),
    position_allocator: &mut PositionAllocator,
) -> Vec<WorldPosition> {
    let (sprite_width, sprite_height, margin) = position_params;

    let mut positions = vec![];
    while positions.len() < quantity {
        positions.push(WorldPosition::alocate_new_position(
            &sprite_width,
            &sprite_height,
            width_in_cells,
            height_in_cells,
            &GRID_CELL_WIDTH,
            &margin,
            position_allocator,
        ));
    }
    positions
}
// pub fn allocate_trees(width_in_cells: &u32, height_in_cells: &u32) -> Vec<WorldPosition> {
//     vec![
//         WorldPosition::alocate_at(
//             &10,
//             &0,
//             &Tree::SPRITE_WIDTH,
//             &Tree::SPRITE_HEIGHT,
//             &&GRID_CELL_WIDTH,
//             &Tree::MARGIN,
//         ),
//         WorldPosition::alocate_at(
//             &10,
//             &1,
//             &Tree::SPRITE_WIDTH,
//             &Tree::SPRITE_HEIGHT,
//             &&GRID_CELL_WIDTH,
//             &Tree::MARGIN,
//         ),
//         WorldPosition::alocate_at(
//             &11,
//             &1,
//             &Tree::SPRITE_WIDTH,
//             &Tree::SPRITE_HEIGHT,
//             &&GRID_CELL_WIDTH,
//             &Tree::MARGIN,
//         ),
//         WorldPosition::alocate_at(
//             &12,
//             &1,
//             &Tree::SPRITE_WIDTH,
//             &Tree::SPRITE_HEIGHT,
//             &&GRID_CELL_WIDTH,
//             &Tree::MARGIN,
//         ),
//         WorldPosition::alocate_at(
//             &12,
//             &0,
//             &Tree::SPRITE_WIDTH,
//             &Tree::SPRITE_HEIGHT,
//             &&GRID_CELL_WIDTH,
//             &Tree::MARGIN,
//         ),
//     ]
// }
