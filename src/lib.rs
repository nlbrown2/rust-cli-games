pub use crate::error::OthelloError;
pub use crate::othello::OthelloGame;
pub use crate::player::HumanPlayer;
use std::collections::VecDeque;

pub trait Game {
    /*
     * Create & initalize a game with players
     */
    fn new(players: VecDeque<Box<dyn Player>>) -> Result<Self, OthelloError>
    where
        Self: Sized;

    /*
     * Blocks until the game is done
     */
    fn run(&mut self) -> Result<(), OthelloError>;
}

pub trait Player: std::fmt::Debug {
    fn new() -> Self
    where
        Self: Sized + std::fmt::Debug;
    fn get_move(&self) -> Result<Pos, OthelloError>;
}

#[derive(Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}
pub mod player;
pub mod error;
pub mod othello;
