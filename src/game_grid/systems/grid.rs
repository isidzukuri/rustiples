use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub use crate::game_grid::ai::*;
pub use crate::game_grid::game_buttons::*;
pub use crate::game_grid::generators::generate_grid;
pub use crate::game_grid::generators::*;
pub use crate::game_grid::graph_node::*;
use crate::game_grid::pathfinding_params::*;

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
            make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RoutePoint);
            make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RouteHead);

            let hero_positions = find_positions_by_type(&game_grid_nodes, GraphNodeType::Hero);
            let axe_positions = find_positions_by_type(&game_grid_nodes, GraphNodeType::Axe);
            println!("hero at: {:?}", hero_positions);

            let mut graph_node_types = vec![
                GraphNodeType::Standard,
                GraphNodeType::RouteHead,
                GraphNodeType::Axe,
            ];

            let pathfinding_params = PathfindingParams {
                start_node: &hero_positions[0],
                end_node: &(col_index, row_index),
                game_grid_nodes: &game_grid_nodes.iter().map(|(_, node)| node).collect(),
                graph_node_types: graph_node_types,
                axe_position: &axe_positions[0],
            };

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
                                node.col == path_node.0
                                    && node.row == path_node.1
                                    && node.node_type != GraphNodeType::Hero
                                    && node.node_type != GraphNodeType::Axe
                            })
                        {
                            // TODO: despawn choped tree sprite which is on top of this grid-sprite
                            sprite.color = Color::PURPLE;
                            node.node_type = GraphNodeType::RoutePoint;
                        };
                    }
                }
            }
        }
    } else if mouse.just_pressed(MouseButton::Left) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
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

            let route_heads = find_positions_by_type(&game_grid_nodes, GraphNodeType::RouteHead);

            // if route_heads.len() > 1 {
            //     let path = find_path(
            //         &route_heads[0],
            //         &route_heads[1],
            //         &game_grid_nodes.iter().map(|(_, node)| node).collect(),
            //         None
            //     );
            //     match path {
            //         None => {}
            //         // None => { print_world_info(commands, "There is no path!!!".to_string()) },
            //         Some(nodes) => {
            //             for path_node in nodes.iter() {
            //                 if let Some((mut sprite, mut node)) =
            //                     game_grid_nodes.iter_mut().find(|(_, ref node)| {
            //                         node.col == path_node.0 && node.row == path_node.1
            //                     })
            //                 {
            //                     sprite.color = Color::PURPLE;
            //                     node.node_type = GraphNodeType::RoutePoint;
            //                 };
            //             }
            //         }
            //     }
            // }
        }
    }

    pub fn make_nodes_standart(
        mut game_grid_nodes: &mut Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
        node_type: GraphNodeType,
    ) {
        for (mut sprite, mut node) in game_grid_nodes.iter_mut() {
            if node.as_mut().node_type == node_type {
                sprite.as_mut().color = Color::GRAY;
                node.as_mut().node_type = GraphNodeType::Standard;
            }
        }
    }

    pub fn detect_graph_node_click(
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

            let col_index = ((position.x) / GRID_CELL_WIDTH).floor() as u32;
            let row_index = ((position.y) / GRID_CELL_WIDTH).floor() as u32;

            println!("{}, {}", row_index, col_index);

            Some((col_index, row_index))
        } else {
            None
        }
    }

    pub fn find_positions_by_type(
        mut game_grid_nodes: &Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
        node_type: GraphNodeType,
    ) -> Vec<(u32, u32)> {
        let mut result = vec![];
        for (_, node) in game_grid_nodes.iter() {
            if node.node_type == node_type {
                result.push((node.col, node.row));
            }
        }
        result
    }

    // pub fn find_node_by_position(
    //     mut game_grid_nodes: &Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
    //     node_type: GraphNodeType,
    // ) -> Vec<(u32, u32)> {
    //     // let mut result = vec![];
    //     // for (_, node) in game_grid_nodes.iter() {
    //     //     if node.node_type == node_type {
    //     //         result.push((node.col, node.row));
    //     //     }
    //     // }
    //     // result
    // }
}
