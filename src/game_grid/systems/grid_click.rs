use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::ai::pathfinding_params::PathfindingParams;
use crate::game_grid::ai::*;
use crate::game_grid::grid::GridEntityType;
use crate::game_grid::grid_entity;
use crate::game_grid::grid_generator::place_entity;
use crate::game_grid::mutation::*;
use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridNode;

use super::GRID_NODE_SIZE;

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GridNode), With<GridNode>>,
    mut grid_entities: Query<
        (Entity, &mut Sprite, &mut GridEntity),
        (With<GridEntity>, Without<GridNode>),
    >,
    mut commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Right) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
            for (mut sprite, node) in game_grid_nodes.iter_mut() {
                if grid.find_entity_type_by_node(&node).is_none() {
                    sprite.color = Color::GRAY;
                }
            } // clean_route();

            let hero_positions = grid.find_coords_by_type(GridEntityType::Hero);
            let axe_positions = grid.find_coords_by_type(GridEntityType::Axe);

            // println!("hero at: {:?}", hero_positions);
            // println!("axe at: {:?}", axe_positions);

            let mut travels_thru = vec![GridEntityType::Axe, GridEntityType::Bridge];

            let mut pathfinding_params = PathfindingParams {
                start_node: hero_positions[0],
                end_node: (col_index, row_index),
                grid: &grid,
                graph_node_types: travels_thru,
                axe_positions: axe_positions,
            };

            // println!("{:?}",find_position_amid(&pathfinding_params, GridEntityType::Tree));
            // let path = find_path(&mut pathfinding_params);
            let mut state = plan_path(pathfinding_params);

            match state.path {
                None => {
                    println!("There is no way")
                }
                // None => { print_world_info(commands, "There is no path!!!".to_string()) },
                Some(nodes) => {
                    for path_node in nodes.iter() {
                        if let Some((mut sprite, mut node)) =
                            game_grid_nodes.iter_mut().find(|(_, ref node)| {
                                (node.x == path_node.0 && node.y == path_node.1)
                                    && (match grid.find_entity_type_by_node(&node) {
                                        Some(entity_type) => {
                                            entity_type != GridEntityType::Hero
                                                && entity_type != GridEntityType::Axe
                                        }
                                        None => true,
                                    })
                            })
                        {
                            sprite.color = Color::PURPLE; //route point

                            if grid.find_entity_type_by_node(&node).is_some() {
                                state.mutations.push(Mutation {
                                    entity_id: None,
                                    mutation_type: MutationType::Destroy,
                                    coords: (node.x, node.y),
                                    entity_type: None,
                                })
                            }
                        };
                    }
                }
            }

            for mutation in state.mutations {
                if mutation.mutation_type == MutationType::Create {
                    place_entity(
                        &mut grid,
                        &mut commands,
                        &asset_server,
                        mutation.entity_type.unwrap(),
                        mutation.coords,
                    );
                }
                if mutation.mutation_type == MutationType::Destroy {
                    let id = grid
                        .find_entry_by_coords(&mutation.coords.0, &mutation.coords.1)
                        .unwrap()
                        .entity_id
                        .unwrap();
                    if let Some((entity, sprite, grid_entity)) = grid_entities
                        .iter()
                        .find(|(_, _, grid_entity)| grid_entity.id == id)
                    {
                        grid.delete_entity(grid_entity.id);
                        commands.entity(entity).despawn();
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
