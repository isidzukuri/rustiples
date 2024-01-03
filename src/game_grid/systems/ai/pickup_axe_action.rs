use super::find_path;
use super::find_path_action::FindPathAction;
use super::Action;
use super::PathfindingParams;
use super::State;
use crate::game_grid::graph_node::*;

pub struct PickupAxeAction {}

impl Action for PickupAxeAction {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        true
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
        let final_destination = params.end_node;

        params.end_node = params.axe_position;

        let path_to_axe = find_path(params);

        if path_to_axe.is_some() {
            params.start_node = params.axe_position;
            params.end_node = final_destination;

            if let Some(ref mut path) = state.path {
                path.append(&mut path_to_axe.unwrap());
            } else {
                state.path = path_to_axe;
            }

            state.actions.push_front(Box::new(FindPathAction {}));
            params.graph_node_types.push(GraphNodeType::Tree);
        }
    }
}
