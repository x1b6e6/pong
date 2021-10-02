pub use super::{ball::Ball, player::Player};

#[derive(Clone, Copy)]
pub struct Progress {
    pub ball: Ball,
    pub player1: Player,
    pub player2: Player,
}

pub trait Drawer {
    fn draw_ball(&mut self, ball: &Ball);
    fn draw_player(&mut self, player: &Player);
}

#[derive(Clone, Copy)]
pub enum Result {
    GameOver(LastGoalFrom),
    GameInProgress(Progress),
    Err,
}

#[derive(Clone, Copy)]
pub enum Status {
    GameOver(LastGoalFrom),
    GameInProgress,
}

#[derive(Clone, Copy)]
pub enum LastGoalFrom {
    Player1,
    Player2,
}
