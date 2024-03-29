use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_grid::ai::pathfinding_params::PathfindingParams;
use crate::game_grid::ai::*;
use crate::game_grid::grid::GridEntityType;

use crate::game_grid::movement_action::MovementAction;
use crate::game_grid::mutation::*;
use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridNode;

use super::GRID_NODE_SIZE;

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    _asset_server: Res<AssetServer>,
    grid: ResMut<Grid>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GridNode), With<GridNode>>,
    _grid_entities: Query<
        (Entity, &mut Sprite, &mut GridEntity),
        (With<GridEntity>, Without<GridNode>),
    >,
    mut commands: Commands,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    if !interaction_query.is_empty() {
        return;
    }

    if mouse.just_pressed(MouseButton::Right) {
        if let Some((col_index, row_index)) = detect_graph_node_click(windows, camera) {
            clear_prev_route_markings(&mut game_grid_nodes, &grid);

            let hero_positions = grid.find_coords_by_type(GridEntityType::Hero);

            let travels_thru = vec![GridEntityType::Axe, GridEntityType::Bridge];

            let pathfinding_params = PathfindingParams {
                start_node: hero_positions[0],
                end_node: (col_index, row_index),
                graph_node_types: travels_thru,
                grid: &grid,
            };

            // println!("{:?}",find_position_amid(&pathfinding_params, GridEntityType::Tree));
            // let path = find_path(&mut pathfinding_params);
            let mut state = plan_path(pathfinding_params);

            match state.path {
                None => {
                    println!("There is no way")
                    // print_world_info(commands, "There is no path!!!".to_string())
                }
                Some(ref _nodes) => {
                    render_route(&mut game_grid_nodes, &grid, &mut state);
                    let grid_entity_id = grid
                        .find_entry_by_coords(&hero_positions[0].0, &hero_positions[0].1)
                        .unwrap()
                        .entity_id
                        .unwrap();
                    // TODO: remove existing MovementAction for entity
                    commands.spawn(MovementAction::new(
                        grid_entity_id,
                        state.path.unwrap(),
                        state.mutations,
                    ));
                }
            }
        }
    } else if mouse.just_pressed(MouseButton::Left) {
        if let Some((_col_index, _row_index)) = detect_graph_node_click(windows, camera) {}
    }
}

fn clear_prev_route_markings(
    game_grid_nodes: &mut Query<(&mut Sprite, &mut GridNode), With<GridNode>>,
    grid: &Grid,
) {
    for (mut sprite, node) in game_grid_nodes.iter_mut() {
        if grid.find_entity_type_by_node(&node).is_none() {
            sprite.color = Color::GRAY;
        }
    }
}

fn render_route(
    game_grid_nodes: &mut Query<(&mut Sprite, &mut GridNode), With<GridNode>>,
    grid: &Grid,
    state: &mut state::State,
) {
    for path_node in state.path.as_ref().unwrap().iter() {
        if let Some((mut sprite, node)) = game_grid_nodes.iter_mut().find(|(_, ref node)| {
            (node.x == path_node.0 && node.y == path_node.1)
                && (match grid.find_entity_type_by_node(&node) {
                    Some(entity_type) => {
                        entity_type != GridEntityType::Hero
                            && entity_type != GridEntityType::Axe
                            && entity_type != GridEntityType::Bridge
                    }
                    None => true,
                })
        }) {
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
        let col_index = ((position.x) / GRID_NODE_SIZE).floor() as u32;
        let row_index = ((position.y) / GRID_NODE_SIZE).floor() as u32;

        println!("{}, {}", col_index, row_index);

        Some((col_index, row_index))
    } else {
        None
    }
}
