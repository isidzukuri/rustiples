use super::find_path;
use super::Action;
use super::PathfindingParams;
use super::PickupAxeAction;
use super::State;
use crate::game_grid::graph_node::*;

pub struct FindPathAction {}

impl Action for FindPathAction {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        true
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        let path = find_path(params);

        if path.is_some() {
            state.destination_reached = true;
            if let Some(ref mut prev_path) = state.path {
                prev_path.append(&mut path.unwrap());
            } else {
                state.path = path;
            }
        } else {
            if !params.graph_node_types.contains(&GraphNodeType::Tree) {
                state.actions.push_front(Box::new(PickupAxeAction {}));
            }
        }
    }
}
