#![allow(unused)]

extern crate tensorflow;

use std::path::PathBuf;
use std::sync::RwLock;

pub mod tree;
use tree::*;

pub mod game;
use game::*;

pub mod alpha_config;
use alpha_config::*;

pub mod network;
use network::*;

pub struct AlphaZero<const N: usize, const I: usize, const O: usize, G: Game<N, I, O>> {
    config: AlphaZeroConfig,
    network_location: NetworkLocation,
    _game_marker: std::marker::PhantomData<G>,
}

impl<const N: usize, const I: usize, const O: usize, G: Game<N, I, O>> AlphaZero<N, I, O, G> {
    fn run_mcts(&self, game_state: G::State) -> G::Action {
        let root = Tree::new(Stats::init(0.0), game_state.perspective());
        Self::evaluate(root.clone(), &game_state, &self.network_location);
        let mut search_path: Vec<Node<N, I, O, G>> = Vec::new();

        for _ in 0..100 {
            let mut node = root.clone();
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

            let value = Self::evaluate(node, &game_state, &self.network_location);
            Self::backpropagate(&search_path, value, game_state.perspective());
        }

        self.select_action(&game_state, root)
    }

    #[allow(non_snake_case)]
    fn select_child(
        &self,
        game_state: &G::State,
        node: Node<N, I, O, G>,
    ) -> (G::Action, Node<N, I, O, G>) {
        let parent_statistics = {
            let node = node.borrow();
            node.statistics.clone()
        };
        let config = &self.config.ucb_parameter;

        let c = f64::log2((1.0 + parent_statistics.visit_count + config.c_base) / config.c_base)
            + config.c_init;
        let SqrtN = f64::sqrt(parent_statistics.visit_count);

        let ucb_score = move |child_statistics: Stats| {
            let n = child_statistics.visit_count;
            let p = child_statistics.prior_probability;
            let u = c * p * SqrtN / (1.0 + n);

            let q = child_statistics.total_action_value / n;

            q + u
        };

        let Ok(result) = Tree::best_child(node, ucb_score) else {
            panic!("No options to select from for leaf Nodes");
        };

        return result;
    }

    ///Select the Action with the highest visit count
    fn select_action(&self, game_state: &G::State, node: Node<N, I, O, G>) -> G::Action {
        fn key(stats: Stats) -> f64 {
            stats.visit_count
        }

        let Ok((result, _)) = Tree::best_child(node, key) else {
            panic!("No options to select from for leaf Nodes");
        };

        return result;
    }

    fn evaluate(node: Node<N, I, O, G>, game_state: &G::State, network: &NetworkLocation) -> f64 {
        let Ok(network_path) = network.read() else {
            panic!("The Lock to the Network has been poisoned.");
        };
        let network_path = network_path.as_path();

        todo!("Actually eval the position");
    }

    fn backpropagate(path: &Vec<Node<N, I, O, G>>, value: f64, perspective: Player) {
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
}

type NetworkLocation = RwLock<PathBuf>;
