use crate::{OthelloError, Player, Pos, UserInput};
#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
}

impl Player for HumanPlayer {
    fn new() -> HumanPlayer {
        HumanPlayer {
            name: String::from("Nathan"),
        }
    }
    fn get_move(&self) -> Result<UserInput, OthelloError> {
        use std::io;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.split(",");
        let x = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        if x == "q" || x == "quit" {
            return Ok(UserInput::Quit);
        }
        let x = x.parse::<usize>()?;
        let y = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        let y = y.parse::<usize>()?;
        return Ok(UserInput::Position(Pos { x, y }));
    }
}
