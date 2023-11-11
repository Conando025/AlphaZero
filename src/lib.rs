#![allow(unused)]

use std::collections::BTreeMap;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::sync::RwLock;
use std::{cell::RefCell, rc::Rc};

pub mod tree;
use tree::*;

pub mod game;
use game::*;

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

pub struct AlphaZeroConfig {
    actor_count: usize,
    ucb_parameter: UCBParameters,
    noise_parameters: NoiseParameters,
    simulation_parameters: SimulationParameters,
    training_parameters: TrainingParameters,
}

impl Default for AlphaZeroConfig {
    fn default() -> Self {
        AlphaZeroConfig {
            actor_count: 5000,
            ucb_parameter: UCBParameters {
                c_base: 19652.0,
                c_init: 1.25,
            },
            noise_parameters: NoiseParameters {
                dirichlet_alpha: 0.3,
                exploration_fraction: 0.25,
            },
            simulation_parameters: SimulationParameters {
                number_of_sampling_moves: 30,
                maximum_number_of_moves: 512,
                number_of_simulations: 800,
            },
            training_parameters: TrainingParameters {
                steps: 700_000,
                checkpoint_interval: 1_000,
                window_size: 1_000_000,
                batch_size: 4096,
                weight_decay: 1e-4,
                momentum: 0.9,
                learning_rate_schedule: vec![
                    (000_000, 2e-1),
                    (100_000, 2e-2),
                    (300_000, 2e-3),
                    (500_000, 2e-4),
                ],
            },
        }
    }
}

struct UCBParameters {
    c_base: f64,
    c_init: f64,
}

struct NoiseParameters {
    dirichlet_alpha: f64,
    exploration_fraction: f64,
}

struct SimulationParameters {
    number_of_sampling_moves: usize,
    maximum_number_of_moves: usize,
    number_of_simulations: usize,
}

struct TrainingParameters {
    steps: usize,
    checkpoint_interval: usize,
    window_size: usize,
    batch_size: usize,

    weight_decay: f64,
    momentum: f64,
    learning_rate_schedule: Vec<(usize, f64)>,
}