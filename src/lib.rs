/*
 * Things to define:
 *  - Traits:
 *      * Game (public)
 *      * Player (public)
 *      * GameBoard (private)
 */
pub use crate::error::OthelloError;

pub trait Game {
    fn new(players: Vec<Box<dyn Player>>) -> Result<Self, OthelloError>
    where
        Self: Sized;
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
pub mod othello;
pub mod player;
pub mod error;
