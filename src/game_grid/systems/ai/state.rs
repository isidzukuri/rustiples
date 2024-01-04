use super::Action;
use std::collections::VecDeque;

pub struct State {
    pub path: Option<Vec<(u32, u32)>>,
    pub cost: Option<f32>,
    pub actions: VecDeque<Box<dyn Action>>,
    pub destination_reached: bool,
}
