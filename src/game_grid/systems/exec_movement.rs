use super::grid_entities_utils::*;
use crate::game_grid::movement_action::MovementAction;
use crate::game_grid::mutation::*;
use crate::game_grid::systems::Grid;
use crate::game_grid::systems::GridEntity;
use crate::game_grid::systems::GridEntityType;
use crate::game_grid::systems::GridNode;

use bevy::prelude::*;

pub fn exec_movement(
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
    mut movements_query: Query<(&mut MovementAction, Entity)>,
    mut grid_entities: Query<
        (Entity, &mut Sprite, &mut GridEntity, &mut Transform),
        (With<GridEntity>, Without<GridNode>),
    >,
    mut commands: Commands,
) {
    if movements_query.is_empty() {
        return;
    }
    for (mut movement, movement_entity) in movements_query.iter_mut() {
        match movement.next_coords() {
            None => commands.entity(movement_entity).despawn(),
            Some(coords) => {
                let next_grid_node_entry = grid.find_entry_by_coords(&coords.0, &coords.1).unwrap();
                if next_grid_node_entry.entity_type == Some(GridEntityType::Tree) {
                    let entity_id_for_mutation = next_grid_node_entry.entity_id.unwrap();

                    if let Some((entity, _, _, _transform)) = grid_entities
                        .iter_mut()
                        .find(|(_, _, grid_entity, _)| grid_entity.id == entity_id_for_mutation)
                    {
                        grid.delete_entity(entity_id_for_mutation);
                        commands.entity(entity).despawn();
                    }
                }

                for mutation in movement
                    .mutations
                    .iter()
                    .filter(|mutation| mutation.coords == coords)
                {
                    if mutation.mutation_type == MutationType::Destroy {
                        if let Some(entity_id_for_mutation) = grid
                            .find_entry_by_coords(&coords.0, &coords.1)
                            .unwrap()
                            .entity_id
                        {
                            if let Some((entity, _, _, _transform)) =
                                grid_entities.iter_mut().find(|(_, _, grid_entity, _)| {
                                    grid_entity.id == entity_id_for_mutation
                                })
                            {
                                grid.delete_entity(entity_id_for_mutation);
                                commands.entity(entity).despawn();
                            }
                        }
                    }
                    if mutation.mutation_type == MutationType::Create {
                        place_entity(
                            &mut grid,
                            &mut commands,
                            &asset_server,
                            mutation.entity_type.unwrap(),
                            mutation.coords,
                        );
                    }
                }

                let (_entity, _sprite, mut grid_entity, mut transform) = grid_entities
                    .iter_mut()
                    .find(|(_, _, grid_entity, _)| grid_entity.id == movement.grid_entity_id)
                    .unwrap();

                let next_grid_node_entry = grid.find_entry_by_coords(&coords.0, &coords.1).unwrap();
                let (window_x, window_y) = node_center_window_coords(&coords);

                if next_grid_node_entry.entity_type.is_none() {
                    grid.move_entity(&grid_entity.id, &coords);
                }
                transform.translation.x = window_x;
                transform.translation.y = window_y;
                grid_entity.x_px = window_x;
                grid_entity.y_px = window_y;
            }
        }
    }
}
