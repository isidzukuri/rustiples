use super::*;

pub struct FindPath {}

impl Action for FindPath {
    fn is_available(&self, _params: &PathfindingParams) -> bool {
        true
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        let path = find_path(params);

        if path.is_some() {
            state.destination_reached = true;
            if let Some(ref mut prev_path) = state.path {
                prev_path.pop();
                prev_path.append(&mut path.unwrap());
            } else {
                state.path = path;
            }
        } else {
            if !params.graph_node_types.contains(&GridEntityType::Tree) {
                state.actions.push_front(Box::new(PickupAxe {}));
            }
        }
    }
}
