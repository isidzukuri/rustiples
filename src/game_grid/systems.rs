use crate::buttons::*;
use bevy::window::PrimaryWindow;
use bevy::{ecs::bundle, prelude::*};
use rand::Rng;
use std::collections::HashMap;
use std::iter;

use super::axe::*;
use super::castle::*;
use super::hero::*;
use super::graph_node::*;
use super::world_position::*;
use super::*;

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

    let mut col_index = 0u32;
    let mut row_index = 0u32;

    let castle_positions = allocate_castles(&width_in_cells, &height_in_cells);
    let heroes_positions = allocate_heroes(&width_in_cells, &height_in_cells);
    let axes_positions = allocate_axes(&width_in_cells, &height_in_cells);
    loop {
        if row_index == height_in_cells && col_index == 0 {
            break;
        }

        let x = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * col_index as f32;
        let y = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * row_index as f32;

        let random_num: u16 = rand::thread_rng().gen_range(1..25);

        let is_castle = castle_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));


        let is_hero = heroes_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        let is_axe = axes_positions
            .iter()
            .any(|position| position.is_owned_cell(&col_index, &row_index));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if is_castle {
                        Color::GOLD
                    } else if is_hero {
                        Color::ANTIQUE_WHITE
                    } else if is_axe {
                        Color::INDIGO
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
        spawn_castle(&mut commands, &asset_server, position)
    }
    for position in heroes_positions {
        spawn_hero(&mut commands, &asset_server, position)
    }
    for position in axes_positions {
        spawn_axe(&mut commands, &asset_server, position)
    }
}

pub fn allocate_heroes(width_in_cells: &u32, height_in_cells: &u32) -> Vec<WorldPosition> {
    vec![WorldPosition::alocate_at(
        &0,
        &0,
        &Hero::SPRITE_WIDTH,
        &Hero::SPRITE_HEIGHT,
        &&GRID_CELL_WIDTH,
        &Hero::MARGIN,
    )]
}

pub fn spawn_hero(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    world_position: WorldPosition,
) {
    let x = (world_position.from_x_cell as f32 * GRID_CELL_WIDTH + world_position.width_px / 2.0)
        as f32;
    let y = (world_position.from_y_cell as f32 * GRID_CELL_WIDTH + world_position.height_px / 2.0)
        as f32;
    let transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: transform,
            texture: asset_server.load("sprites/hero.png"),
            ..default()
        },
        Hero {
            world_position: world_position,
        },
    ));
}


pub fn allocate_axes(width_in_cells: &u32, height_in_cells: &u32) -> Vec<WorldPosition> {
    vec![WorldPosition::alocate_at(
        &0,
        &1,
        &Axe::SPRITE_WIDTH,
        &Axe::SPRITE_HEIGHT,
        &&GRID_CELL_WIDTH,
        &Axe::MARGIN,
    )]
}

pub fn spawn_axe(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    world_position: WorldPosition,
) {
    let x = (world_position.from_x_cell as f32 * GRID_CELL_WIDTH + world_position.width_px / 2.0)
        as f32;
    let y = (world_position.from_y_cell as f32 * GRID_CELL_WIDTH + world_position.height_px / 2.0)
        as f32;
    let transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: transform,
            texture: asset_server.load("sprites/axe.png"),
            ..default()
        },
        Axe {
            world_position: world_position,
        },
    ));
}



pub fn allocate_castles(width_in_cells: &u32, height_in_cells: &u32) -> Vec<WorldPosition> {
    let mut castle_positions = vec![];
    let mut generations_count = 0;
    for num in 0..2 {
        let mut castle_position = WorldPosition::alocate_new_position(
            &Castle::SPRITE_WIDTH,
            &Castle::SPRITE_HEIGHT,
            width_in_cells,
            height_in_cells,
            &GRID_CELL_WIDTH,
            &Castle::MARGIN,
        );

        while castle_positions
            .iter()
            .any(|position| castle_position.intersects_with(position))
        {
            castle_position = WorldPosition::alocate_new_position(
                &Castle::SPRITE_WIDTH,
                &Castle::SPRITE_HEIGHT,
                &width_in_cells,
                &height_in_cells,
                &&GRID_CELL_WIDTH,
                &Castle::MARGIN,
            );
            generations_count += 1;
            if generations_count == 25 {
                panic!("world is to small to fit all castles")
            }
        }
        castle_positions.push(castle_position);
    }
    castle_positions
}

pub fn spawn_castle(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    world_position: WorldPosition,
) {
    let x = (world_position.from_x_cell as f32 * GRID_CELL_WIDTH + world_position.width_px / 2.0)
        as f32;
    let y = (world_position.from_y_cell as f32 * GRID_CELL_WIDTH + world_position.height_px / 2.0)
        as f32;
    let transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: transform,
            texture: asset_server.load("sprites/castle.png"),
            ..default()
        },
        Castle {
            world_position: world_position,
        },
    ));
}

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
) {
    let window = windows.single();

    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        if let Some(position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // println!("{:?}", position);

            let col_index = ((position.x) / GRID_CELL_WIDTH).floor() as u32;
            let row_index = ((position.y) / GRID_CELL_WIDTH).floor() as u32;
            println!("{}, {}", row_index, col_index);

            if let Some((mut sprite, mut node)) = game_grid_nodes
                .iter_mut()
                .find(|(_, ref node)| node.row == row_index && node.col == col_index)
            {
                if node.node_type != GraphNodeType::Standard {
                    return;
                }
                // println!("{:?}", sprite);
                sprite.color = Color::GREEN;
                node.node_type = GraphNodeType::RouteHead;
                // println!("{:?}", node);
            };

            let mut route_heads = vec![];
            for (_, node) in game_grid_nodes.iter() {
                if node.node_type == GraphNodeType::RouteHead {
                    route_heads.push((node.col, node.row));
                }
            }

            if route_heads.len() > 1 {
                let path = find_path(
                    &route_heads[0],
                    &route_heads[1],
                    &game_grid_nodes.iter().map(|(_, node)| node).collect(),
                );
                match path {
                    None => {}
                    // None => { print_world_info(commands, "There is no path!!!".to_string()) },
                    Some(nodes) => {
                        for path_node in nodes.iter() {
                            if let Some((mut sprite, mut node)) =
                                game_grid_nodes.iter_mut().find(|(_, ref node)| {
                                    node.col == path_node.0 && node.row == path_node.1
                                })
                            {
                                sprite.color = Color::PURPLE;
                                node.node_type = GraphNodeType::RoutePoint;
                            };
                        }
                    }
                }
            }
        }
    }
}

// (col, row)
pub fn find_path(
    start_node: &(u32, u32),
    end_node: &(u32, u32),
    game_grid_nodes: &Vec<&GraphNode>,
) -> Option<Vec<(u32, u32)>> {
    let mut open_set: Vec<(u32, u32)> = vec![];
    open_set.push(*start_node);

    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut g_score: HashMap<(u32, u32), f32> = HashMap::new();
    g_score.insert(*start_node, 0.0);

    let mut f_score: HashMap<(u32, u32), f32> = HashMap::new();
    f_score.insert(*start_node, 0.0);

    while open_set.len() > 0 {
        let mut current = open_set[0];

        for node in open_set.iter() {
            if let Some(score) = f_score.get(node) {
                if score < f_score.get(&current).unwrap() {
                    current = *node;
                }
            }
        }

        if &current == end_node {
            println!(
                "------- A* score: {} --------",
                f_score.get(&current).unwrap()
            );

            return Some(reconstruct_path(came_from, end_node));
        }

        let remove_index = open_set.iter().position(|item| *item == current).unwrap();
        open_set.remove(remove_index);

        let mut connections: Vec<(u32, u32)> = vec![];

        connections.push((current.0 + 1, current.1));
        if current.0 > 0 {
            connections.push((current.0 - 1, current.1))
        };
        connections.push((current.0, current.1 + 1));
        if current.1 > 0 {
            connections.push((current.0, current.1 - 1))
        };

        for neighbor in connections.iter() {
            if game_grid_nodes
                .iter()
                .find(|node| {
                    (node.col == neighbor.0 && node.row == neighbor.1)
                        && (node.node_type == GraphNodeType::Standard
                            || node.node_type == GraphNodeType::RouteHead)
                })
                .is_none()
            {
                continue;
            };

            let tentative_g_score = g_score.get(&current).unwrap() + 1.0;
            if &tentative_g_score < g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(*neighbor, current.clone());
                g_score.insert(*neighbor, tentative_g_score);

                let f_score_neighbor = tentative_g_score;
                f_score.insert(*neighbor, f_score_neighbor);

                if !open_set.contains(&&neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
    }

    None
}

pub fn reconstruct_path(
    came_from: HashMap<(u32, u32), (u32, u32)>,
    end: &(u32, u32),
) -> Vec<(u32, u32)> {
    let mut total_path = vec![];

    total_path.push(*end);

    let mut current = end;

    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap();

        total_path.insert(0, *current);
    }

    total_path
}

pub fn spawn_control_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<Entity, With<Menu>>,
) {
    spawn_button(
        "Save map".to_string(),
        "export_grid".to_string(),
        commands,
        asset_server,
        query,
    );
}

pub fn button_pressed_event_listener(mut listener: EventReader<ButtonPressedEvent>) {
    for event in listener.read() {
        if event.event_type == "export_grid".to_string() {
            println!("Grid entity exported to");
        }
    }
}
