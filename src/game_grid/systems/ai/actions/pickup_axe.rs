use super::*;

pub struct PickupAxe {}

impl Action for PickupAxe {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        params.axe_positions.len() > 0
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        let final_destination = params.end_node;

        let axe_position = params.axe_positions.get(0).unwrap().clone();
        params.end_node = axe_position;

        let path_to_axe = find_path(params);

        if path_to_axe.is_some() {
            params.start_node = axe_position;

            if let Some(ref mut path) = state.path {
                path.append(&mut path_to_axe.unwrap());
            } else {
                state.path = path_to_axe;
            }

            // state.mutations.push(Mutation {
            //     entity_id: None,
            //     mutation_type: MutationType::Destroy,
            //     coords: axe_position,
            //     entity_type: None
            // });
            state.actions.push_front(Box::new(BuildLumberMill {}));
            state.actions.push_front(Box::new(FindPath {}));
            params.graph_node_types.push(GridEntityType::Tree);
        }
        params.end_node = final_destination;
    }
}
