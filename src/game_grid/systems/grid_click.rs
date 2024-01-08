use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::ai::pathfinding_params::PathfindingParams;
use crate::game_grid::ai::*;
use crate::game_grid::grid::GridEntityType;
use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridNode;

use super::GRID_NODE_SIZE;

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    grid: ResMut<Grid>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GridNode), With<GridNode>>,
    mut grid_entities: Query<(&mut Sprite, &mut GridEntity), (With<GridEntity>, Without<GridNode>)>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
            //         make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RoutePoint);
            //         make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RouteHead);

            let hero_positions = grid.find_coords_by_type(GridEntityType::Hero);
            let axe_positions = grid.find_coords_by_type(GridEntityType::Axe);

            println!("hero at: {:?}", hero_positions);
            println!("axe at: {:?}", axe_positions);

            let mut graph_node_types = vec![
                // GridEntityType::Standard,
                // GridEntityType::RouteHead,
                GridEntityType::Axe,
                // GridEntityType::Mineral,
            ];

            let mut pathfinding_params = PathfindingParams {
                start_node: &hero_positions[0],
                end_node: &(col_index, row_index),
                grid: &grid,
                graph_node_types: graph_node_types,
                axe_position: &axe_positions[0],
            };
            // let path = find_path(&mut pathfinding_params);
            let path = plan_path(pathfinding_params);

            match path {
                None => {
                    println!("There is no way")
                }
                // None => { print_world_info(commands, "There is no path!!!".to_string()) },
                Some(nodes) => {
                    for path_node in nodes.iter() {
                        if let Some((mut sprite, mut node)) =
                            game_grid_nodes.iter_mut().find(|(_, ref node)| {
                                (node.x == path_node.0 && node.y == path_node.1) &&
                                (
                                    match grid.find_entity_type_by_node(&node) {
                                        Some(entity_type) => {
                                            entity_type != GridEntityType::Hero
                                                && entity_type != GridEntityType::Axe
                                        }
                                        None => true,
                                    })
                            })
                        {
                            sprite.color = Color::PURPLE;
                            // node.node_type = GridEntityType::RoutePoint;
                        };
                    }
                }
            }
        }
    } else if mouse.just_pressed(MouseButton::Left) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
            //         if let Some((mut sprite, mut node)) = game_grid_nodes
            //             .iter_mut()
            //             .find(|(_, ref node)| node.row == row_index && node.col == col_index)
            //         {
            //             if node.node_type != GraphNodeType::Standard {
            //                 return;
            //             }
            //             // println!("{:?}", sprite);
            //             sprite.color = Color::GREEN;
            //             node.node_type = GraphNodeType::RouteHead;
            //             // println!("{:?}", node);
            //         };

            //         let route_heads = find_positions_by_type(&game_grid_nodes, GraphNodeType::RouteHead);

            //         // if route_heads.len() > 1 {
            //         //     let path = find_path(
            //         //         &route_heads[0],
            //         //         &route_heads[1],
            //         //         &game_grid_nodes.iter().map(|(_, node)| node).collect(),
            //         //         None
            //         //     );
            //         //     match path {
            //         //         None => {}
            //         //         // None => { print_world_info(commands, "There is no path!!!".to_string()) },
            //         //         Some(nodes) => {
            //         //             for path_node in nodes.iter() {
            //         //                 if let Some((mut sprite, mut node)) =
            //         //                     game_grid_nodes.iter_mut().find(|(_, ref node)| {
            //         //                         node.col == path_node.0 && node.row == path_node.1
            //         //                     })
            //         //                 {
            //         //                     sprite.color = Color::PURPLE;
            //         //                     node.node_type = GraphNodeType::RoutePoint;
            //         //                 };
            //         //             }
            //         //         }
            //         //     }
            //         // }
        }
    }
}

fn detect_graph_node_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) -> Option<(u32, u32)> {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // println!("{:?}", position);

        let col_index = ((position.x) / GRID_NODE_SIZE).floor() as u32;
        let row_index = ((position.y) / GRID_NODE_SIZE).floor() as u32;

        println!("{}, {}", col_index, row_index);

        Some((col_index, row_index))
    } else {
        None
    }
}
