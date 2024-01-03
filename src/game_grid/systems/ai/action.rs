use super::PathfindingParams;
use super::State;

pub trait Action {
    fn is_available(&self, params: &PathfindingParams) -> bool; //TODO: decouple actions with this
    fn exec(&self, params: &mut PathfindingParams, state: &mut State);
}
