pub trait Game<const N: usize, const I: usize, const O: usize> {
    type Action: PartialEq + Eq + PartialOrd + Ord + Clone;
    type State: GameState<Self::Action> + Clone;
    #[allow(non_upper_case_globals)]
    const InputShape: [usize; 3] = [N, N, I];
    #[allow(non_upper_case_globals)]
    const OutputShape: [usize; 3] = [N, N, O];

    fn image(&self) -> [[[f32; N]; N]; I];
}

pub trait GameState<Action> {
    fn apply(&mut self, action: Action);

    fn perspective(&self) -> Player;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Player {
    Left,
    Right,
}
