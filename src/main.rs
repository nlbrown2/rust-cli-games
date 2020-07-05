use rust_othello::{Game, OthelloError, Player, OthelloGame, HumanPlayer};
use std::collections::VecDeque;

fn main() -> Result<(), OthelloError> {
    let mut players : VecDeque<Box<dyn Player>> = VecDeque::new();
    players.push_back(Box::new(HumanPlayer::new()));
    players.push_back(Box::new(HumanPlayer::new()));
    let mut g = OthelloGame::new(players)?;
    g.run()?;
    Ok(())
}
