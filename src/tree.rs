use super::*;

pub struct Tree {
    pub statistics: Stats,
    pub to_play: Player,
    pub children: Option<BTreeMap<Action, Node>>,
}

impl Tree {
    pub fn new(statistics: Stats, to_play: Player) -> Node {
        Rc::new(RefCell::new(Tree {
            statistics,
            to_play,
            children: None,
        }))
    }
}

pub type Node = Rc<RefCell<Tree>>;

#[derive(Clone)]
pub struct Stats {
    pub visit_count: f64,
    pub total_action_value: f64,
    pub prior_probability: f64,
}

impl Stats {
    pub fn init(prior: f64) -> Self {
        Self {
            visit_count: 0.0,
            total_action_value: 0.0,
            prior_probability: prior,
        }
    }
}
