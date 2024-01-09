use super::actions;
use crate::game_grid::ai::Mutation;
use std::collections::VecDeque;

pub struct State {
    pub path: Option<Vec<(u32, u32)>>,
    pub cost: Option<f32>,
    pub actions: VecDeque<Box<dyn actions::Action>>,
    pub destination_reached: bool,
    pub mutations: Vec<Mutation>,
}

impl State {
    pub fn new() -> Self {
        Self {
            path: None,
            cost: None,
            actions: VecDeque::new(),
            destination_reached: false,
            mutations: vec![],
        }
    }
}
