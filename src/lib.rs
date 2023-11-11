#![allow(unused)]

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::RwLock;
use std::{cell::RefCell, rc::Rc};

pub mod tree;
use tree::*;

pub mod game;
use game::*;

pub mod alpha_config;
use alpha_config::*;

pub struct AlphaZero {
    config: AlphaZeroConfig,
    network_location: NetworkLocation,
}

impl AlphaZero {
    fn run_mcts(&self, game_state: GameState) -> Action {
        let root: Node = Tree::new(Stats::init(0.0), game_state.perspective());
        evaluate(root.clone(), &game_state, &self.network_location);
        let mut search_path: Vec<Rc<RefCell<Tree>>> = Vec::new();

        for _ in 0..100 {
            let mut node: Node = root.clone();
            let mut game_state = game_state.clone();
            search_path.clear();
            search_path.push(root.clone());

            let mut is_leaf = root.borrow().children.is_none();

            while !is_leaf {
                let (action, selected_node) = self.select_child(&game_state, root.clone());
                game_state.apply(action);
                node = selected_node.clone();
                search_path.push(node.clone());
                is_leaf = node.borrow().children.is_none();
            }

            let value = evaluate(node, &game_state, &self.network_location);
            backpropagate(&search_path, value, game_state.perspective());
        }

        self.select_action(&game_state, root)
    }

    #[allow(non_snake_case)]
    fn select_child(&self, game_state: &GameState, node: Node) -> (Action, Node) {
        let mut best: Option<(f64, (Action, Node))> = None;
        let node = node.borrow();
        let Some(action_map) = node.children.as_ref() else {
            panic!("Should not have been called on a leaf node");
        };
        let parent_statistics = node.statistics.clone();
        let config = &self.config.ucb_parameter;

        let C = f64::log2((1.0 + parent_statistics.visit_count + config.c_base) / config.c_base)
            + config.c_init;
        let SqrtN = f64::sqrt(parent_statistics.visit_count);

        for (action, node) in action_map.iter() {
            let child_statistics = node.borrow().statistics.clone();

            let N = child_statistics.visit_count;
            let P = child_statistics.prior_probability;
            let U = C * P * SqrtN / (1.0 + N);

            let Q = child_statistics.total_action_value / N;

            let metric = Q + U;
            let Some((best_metric, _)) = &best else {
                best = Some((metric, (action.clone(), node.clone())));
                continue;
            };

            if metric > *best_metric {
                best = Some((metric, (action.clone(), node.clone())));
            }
        }

        let Some((_, result)) = best else {
            panic!("No options to select from for leaf Nodes");
        };

        return result;
    }

    ///Select the Action with the highest visit count
    fn select_action(&self, game_state: &GameState, node: Node) -> Action {
        let mut best: Option<(f64, (Action, Node))> = None;
        let node = node.borrow();
        let Some(action_map) = node.children.as_ref() else {
            panic!("Should not have been called on a leaf node");
        };
        for (action, node) in action_map.iter() {
            let child_statistics = node.borrow().statistics.clone();

            let metric = child_statistics.visit_count;
            let Some((best_metric, _)) = &best else {
                best = Some((metric, (action.clone(), node.clone())));
                continue;
            };

            if metric > *best_metric {
                best = Some((metric, (action.clone(), node.clone())));
            }
        }

        let Some((_, (result, _))) = best else {
            panic!("No options to select from for leaf Nodes");
        };

        return result;
    }
}

fn evaluate(node: Node, game_state: &GameState, network: &NetworkLocation) -> f64 {
    let Ok(network_path) = network.read() else {
        panic!("The Lock to the Network has been poisoned.");
    };
    let network_path = network_path.as_path();

    todo!("Actually eval the position");
}

fn backpropagate(path: &Vec<Node>, value: f64, perspective: Player) {
    for node in path {
        let mut node = node.borrow_mut();
        let node_perspective = { node.to_play };
        let stats = &mut node.statistics;

        stats.visit_count += 1.0;
        stats.total_action_value += {
            if perspective == node_perspective {
                value
            } else {
                1.0 - value
            }
        };
    }
}

type NetworkLocation = RwLock<PathBuf>;