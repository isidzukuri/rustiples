use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::collections::HashMap;

use super::{GraphNode, GraphNodeType};

pub const GRID_CELL_WIDTH: f32 = 50.0 as f32;
pub const HALF_GRID_CELL_WIDTH: f32 = 25.0 as f32;

pub fn generate_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let width_in_cells = (window.width() / GRID_CELL_WIDTH) as u32;
    let height_in_cells = (window.height() / GRID_CELL_WIDTH) as u32;

    let mut col_index = 0u32;
    let mut row_index = 0u32;
    loop {
        if row_index == height_in_cells && col_index == 0 {
            break;
        }

        let x = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * col_index as f32;
        let y = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * row_index as f32;

        let random_num: u16 = rand::thread_rng().gen_range(1..5);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if random_num == 1 {
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
                node_type: if random_num == 1 {
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
