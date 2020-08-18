use crate::board::BOARD_WIDTH;
pub use crate::error::OthelloError;
pub use crate::othello::OthelloGame;
pub use crate::player::HumanPlayer;
use std::collections::VecDeque;
use std::convert::TryInto;

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
    fn get_move(&self) -> Result<UserInput, OthelloError>;
}

#[derive(Debug)]
pub enum UserInput {
    Position(Pos),
    Quit,
}

impl From<Pos> for UserInput {
    fn from(pos: Pos) -> UserInput {
        UserInput::Position(pos)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    pub fn new(row: i32, col: i32) -> Result<Self, OthelloError> {
        let (row, col) = Pos::cast(row, col)?;
        Ok(Pos { row, col })
    }
    pub fn shift(&mut self, dr: i32, dc: i32) -> Result<(), OthelloError> {
        // self.{row, col} should never overflow as it is limited by BOARD_WIDTH
        let row = self.row as i32 + dr;
        let col = self.col as i32 + dc;
        let (row, col) = Pos::cast(row, col)?;
        self.row = row;
        self.col = col;
        Ok(())
    }
    fn cast(row: i32, col: i32) -> Result<(usize, usize), OthelloError> {
        let row: usize = row.try_into().map_err(|_| OthelloError::IllegalMove)?;
        let col: usize = col.try_into().map_err(|_| OthelloError::IllegalMove)?;
        if row >= BOARD_WIDTH || col >= BOARD_WIDTH {
            return Err(OthelloError::IllegalMove);
        }
        Ok((row, col))
    }
}
pub mod board;
pub mod error;
pub mod othello;
pub mod player;
