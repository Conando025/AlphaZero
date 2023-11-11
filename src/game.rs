#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Action {}

#[derive(Clone)]
pub struct GameState {}

impl GameState {
    pub fn apply(&mut self, action: Action) {
        todo!("Apply the action");
    }

    pub fn perspective(&self) -> Player {
        todo!();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Player {
    Left,
    Right,
}
