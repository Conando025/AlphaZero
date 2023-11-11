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

    /**Select the child node the maximises some key,
     * 
     * #Errors
     * If the given Node has no children the a LeafNode error will be returned
     */
    pub fn best_child<Key>(node: Node, key: Key) -> Result<(Action, Node), LeafNode>
    where Key: Fn(Stats) -> f64,
    {
        let mut best: Option<(f64, (Action, Node))> = None;
        let node = node.borrow();
        let Some(action_map) = node.children.as_ref() else {
            return Err(LeafNode);
        };
        for (action, node) in action_map.iter() {
            let child_statistics = node.borrow().statistics.clone();

            let metric = key(child_statistics);
            let Some((best_metric, _)) = &best else {
                best = Some((metric, (action.clone(), node.clone())));
                continue;
            };

            if metric > *best_metric {
                best = Some((metric, (action.clone(), node.clone())));
            }
        }
        let Some((_, result)) = best else {
            return Err(LeafNode);
        };

        Ok(result)
    }
}

pub struct LeafNode;

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
