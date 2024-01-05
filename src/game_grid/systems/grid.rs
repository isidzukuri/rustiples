use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::ai::pathfinding_params::*;
pub use crate::game_grid::ai::*;
pub use crate::game_grid::game_buttons::*;
pub use crate::game_grid::generators::generate_grid;
pub use crate::game_grid::generators::*;
pub use crate::game_grid::graph_node::*;
use crate::game_grid::world_position::WorldPosition;

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
                GraphNodeType::Mineral,
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

            println!("{}, {}", col_index, row_index);

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

use rand::prelude::SliceRandom;
#[derive(Copy, Clone, Debug)]
pub struct PositionAllocation {
    pub from_x_cell: u32,
    pub from_y_cell: u32,
    pub to_x_cell: u32,
    pub to_y_cell: u32,
}
pub struct PositionAllocator {
    pub width_cells: u32,
    pub height_cells: u32,
    pub reserved_cells: Vec<(u32, u32)>, // pub reserved_positions: Vec<WorldPosition>
}

impl PositionAllocator {
    pub fn reserve(&mut self, col_index: u32, row_index: u32) {
        let node = (col_index, row_index);
        if self.reserved_cells.contains(&node) {
            panic!("Position already reserved");
        }

        self.reserved_cells.push(node);
    }

    pub fn allocate(
        &mut self,
        width_cells: u32,
        height_cells: u32,
    ) -> Option<(PositionAllocation)> {
        let mut col_index: u32 = 0;
        let mut row_index: u32 = 0;
        let mut variants = vec![];

        while true {
            if row_index == self.height_cells && col_index == 0 {
                break;
            }
            if (col_index + width_cells) < self.width_cells
                && (row_index + height_cells) < self.height_cells
            {
                let mut allocated = true;

                for cur_col_index in (col_index..(col_index + width_cells)) {
                    for cur_row_index in (row_index..(row_index + height_cells)) {
                        if self.reserved_cells.contains(&(cur_col_index, row_index)) {
                            allocated = false;
                            break;
                        }
                    }
                    if allocated == false {
                        break;
                    }
                }

                if allocated {
                    variants.push(PositionAllocation {
                        from_x_cell: col_index,
                        from_y_cell: row_index,
                        to_x_cell: col_index + width_cells - 1,
                        to_y_cell: row_index + height_cells - 1,
                    });
                }
            }

            col_index += 1;
            if col_index == self.width_cells {
                col_index = 0;
                row_index += 1;
            };
        }

        match variants.choose(&mut rand::thread_rng()) {
            None => None,
            Some(allocation) => {
                for cur_col_index in (allocation.from_x_cell..allocation.to_x_cell) {
                    for cur_row_index in (allocation.from_y_cell..allocation.to_y_cell) {
                        self.reserved_cells.push((cur_col_index, cur_row_index));
                    }
                }

                Some(*allocation)
            }
        }
    }
}
