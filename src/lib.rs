use std::collections::BTreeMap;
use std::{rc::Rc, cell::RefCell};


#[derive(Default)]
pub struct Tree {
    statistics: Stats,
    children: Option<BTreeMap<Action, Node>>,
}

type Node = Rc<RefCell<Tree>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {}

#[derive(Default)]
pub struct Stats {
    visit_count: u64,
    total_action_value: f64,
    mean_action_value: f64,
    prior_probability: f64,
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
        let root: Node = Rc::new(RefCell::new(Tree::default()));
        evaluate(root.clone(), &game_state);
        let mut search_path: Vec<Rc<RefCell<Tree>>> = Vec::new();

        for _  in 0..100 {
            let mut node: Node = root.clone();
            let mut game_state = game_state.clone();
            search_path.clear();
            search_path.push(root.clone());

            let mut is_leaf = root.borrow().children.is_none();

            while !is_leaf {
                let action = self.select_action(&game_state, root.clone());
                let selected_node: Node = {
                    let node = node.borrow();
                    let map =  node.children.as_ref().expect("It isn't a Leaf");
                    map.get(&action).expect("One of the children should have been selected").clone()
                };
                game_state.apply(action);
                node = selected_node.clone();
                search_path.push(node.clone());
                is_leaf = node.borrow().children.is_none();
            }

            let value = evaluate(node, &game_state);
            backpropagate(&search_path, value, game_state.perspective());
        }

        return self.select_action(&game_state, root);
    }

    fn select_child(&self) -> (Action, Node) {
        todo!()
    }

    fn select_action(&self, game_state: &GameState, node: Node) -> Action {
        todo!()
    }
}

fn evaluate(node: Node, game_state: &GameState) -> f64 {
    todo!("Actually eval the position");
}

fn backpropagate(path: &Vec<Node>, value: f64, perspective: Player) {
    todo!("backpropagate")
}

pub struct AlphaZeroConfig {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Left,
    Right,
}