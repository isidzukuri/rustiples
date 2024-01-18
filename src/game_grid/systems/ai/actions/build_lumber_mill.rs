use super::*;

pub struct BuildLumberMill {}

impl Action for BuildLumberMill {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        params.graph_node_types.contains(&GridEntityType::Tree)
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        let lumber_position = find_position_amid(params, GridEntityType::Tree).unwrap();
        let final_destination = params.end_node;

        params.end_node = lumber_position;

        let path_to_point = find_path(params);

        if path_to_point.is_some() {
            params.start_node = lumber_position;

            if let Some(ref mut path) = state.path {
                path.pop();
                path.append(&mut path_to_point.unwrap());
            } else {
                state.path = path_to_point;
            }

            state.mutations.push(Mutation {
                entity_id: None,
                mutation_type: MutationType::Create,
                coords: lumber_position,
                entity_type: Some(GridEntityType::LumberMill),
            });
            state.actions.push_back(Box::new(BuildBridge {}));
        }
        params.end_node = final_destination;
    }
}
