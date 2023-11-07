use std::collections::BTreeMap;
use std::mem::MaybeUninit;
use std::{cell::RefCell, rc::Rc};

pub struct Tree {
    statistics: Stats,
    children: Option<BTreeMap<Action, Node>>,
}

type Node = Rc<RefCell<Tree>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Action {}

#[derive(Clone)]
pub struct Stats {
    visit_count: f64,
    total_action_value: f64,
    mean_action_value: f64,
    prior_probability: f64,
}

impl Stats {
    fn init(prior: f64) -> Self {
        Self {
            visit_count: 0.0,
            total_action_value: 0.0,
            mean_action_value: 0.0,
            prior_probability: prior,
        }
    }
}

#[derive(Clone)]
pub struct GameState {}

impl GameState {
    fn apply(&mut self, action: Action) {
        todo!("Apply the action");
    }

    fn perspective(&self) -> Player {
        todo!();
    }
}

pub struct AlphaZero {
    config: AlphaZeroConfig,
}

impl AlphaZero {
    fn run_mcts(&self, game_state: GameState) -> Action {
        let root: Node = Rc::new(RefCell::new(Tree {
            statistics: Stats::init(0.0),
            children: None,
        }));
        evaluate(root.clone(), &game_state);
        let mut search_path: Vec<Rc<RefCell<Tree>>> = Vec::new();

        for _ in 0..100 {
            let mut node: Node = root.clone();
            let mut game_state = game_state.clone();
            search_path.clear();
            search_path.push(root.clone());

            let mut is_leaf = root.borrow().children.is_none();

            while !is_leaf {
                let (action, selected_node) = self.select_action(&game_state, root.clone());
                game_state.apply(action);
                node = selected_node.clone();
                search_path.push(node.clone());
                is_leaf = node.borrow().children.is_none();
            }

            let value = evaluate(node, &game_state);
            backpropagate(&search_path, value, game_state.perspective());
        }

        let (choosen_action, _) = self.select_action(&game_state, root);
        return choosen_action;
    }

    #[allow(non_snake_case)]
    fn select_action(&self, game_state: &GameState, node: Node) -> (Action, Node) {
        let mut best: Option<(f64, (Action, Node))> = None;
        let node = node.borrow();
        let Some(action_map) = node.children.as_ref() else {
            panic!("Should not have been called on a leaf node");
        };
        let parent_statistics = node.statistics.clone();
        let config = &self.config;

        let C = f64::log2((1.0 + parent_statistics.visit_count + config.c_base) / config.c_base)
            + config.c_init;
        let SqrtN = f64::sqrt(parent_statistics.visit_count);

        for (action, node) in action_map.iter() {
            let child_statistics = node.borrow().statistics.clone();

            let N = child_statistics.visit_count;
            let P = child_statistics.prior_probability;
            let U = C * P * SqrtN / (1.0 + N);

            let Q = child_statistics.mean_action_value;

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
            panic!("No option to select for leaf Nodes");
        };

        return result;
    }
}

fn evaluate(node: Node, game_state: &GameState) -> f64 {
    todo!("Actually eval the position");
}

fn backpropagate(path: &Vec<Node>, value: f64, perspective: Player) {
    todo!("backpropagate")
}

pub struct AlphaZeroConfig {
    c_base: f64,
    c_init: f64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Left,
    Right,
}
