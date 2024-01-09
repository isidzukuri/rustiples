use super::*;

pub struct BuildBridge {}

impl Action for BuildBridge {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        true
        // if has lumber mill
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        params.graph_node_types.push(GridEntityType::Water);
        match find_path(params) {
            Some(path) => {
                let bridge_positions = path.iter().filter(|node| {
                    params.grid.find_entity_type_by_coords(&node.0, &node.1)
                        == Some(GridEntityType::Water)
                });

                for bridge_position in bridge_positions {
                    state.mutations.push(Mutation {
                        entity_id: None,
                        mutation_type: MutationType::Destroy,
                        coords: *bridge_position,
                        entity_type: None,
                    });
                    state.mutations.push(Mutation {
                        entity_id: None,
                        mutation_type: MutationType::Create,
                        coords: *bridge_position,
                        entity_type: Some(GridEntityType::Bridge),
                    });
                }
                state.actions.push_front(Box::new(FindPath {}));
            }
            None => {
                params
                    .graph_node_types
                    .retain(|&item| item != GridEntityType::Water);
            }
        }
    }
}
