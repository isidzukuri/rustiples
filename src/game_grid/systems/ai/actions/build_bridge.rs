use super::*;

pub struct BuildBridge {}

impl Action for BuildBridge {
    fn is_available(&self, params: &PathfindingParams) -> bool {
        true
        // if has lumber mill
    }

    fn exec(&self, params: &mut PathfindingParams, state: &mut State) {
    }
}
