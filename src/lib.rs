/*
 * Things to define:
 *  - Traits:
 *      * Game (public)
 *      * Player (public)
 *      * GameBoard (private)
 */
use std::error::Error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum OthelloError {
    InvalidArgs,
    Fmt(fmt::Error),
    IllegalMove,
    IOError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for OthelloError {
    fn from(error: io::Error) -> OthelloError {
        OthelloError::IOError(error)
    }
}

impl From<num::ParseIntError> for OthelloError {
    fn from(error: num::ParseIntError) -> OthelloError {
        OthelloError::ParseError(error)
    }
}

impl Error for OthelloError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OthelloError::Fmt(err) => Some(err),
            OthelloError::IOError(err) => Some(err),
            OthelloError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for OthelloError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OthelloError::InvalidArgs => write!(f, "Invalid arguments."),
            OthelloError::Fmt(err) => write!(f, "{}", err),
            OthelloError::IllegalMove => write!(f, "You can't move there!"),
            OthelloError::IOError(err) => write!(f, "{}", err),
            OthelloError::ParseError(err) => write!(f, "{}", err),
        }
    }
}

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
