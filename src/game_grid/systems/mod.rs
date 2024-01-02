pub mod ai;
pub mod game_buttons;
pub mod generators;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub use crate::game_grid::ai::*;
pub use crate::game_grid::game_buttons::*;
pub use crate::game_grid::generators::generate_grid;
pub use crate::game_grid::generators::*;
pub use crate::game_grid::graph_node::*;

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
) {
    let window = windows.single();

    if mouse.just_pressed(MouseButton::Right) {
        make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RoutePoint);
        make_nodes_standart(&mut game_grid_nodes, GraphNodeType::RouteHead);
    } else if mouse.just_pressed(MouseButton::Left) {
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

    pub fn make_nodes_standart(
        mut game_grid_nodes: &mut Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
        node_type: GraphNodeType
    ) {
        for (mut sprite, mut node) in game_grid_nodes.iter_mut() {
            if node.as_mut().node_type == node_type {
                sprite.as_mut().color = Color::GRAY;
                node.as_mut().node_type = GraphNodeType::Standard;
            }
        }

    }

}
