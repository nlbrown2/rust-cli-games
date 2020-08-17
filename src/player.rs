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
        let row = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        if row == "q" || row == "quit" {
            // TODO: normalize input so things like Quit or Q also work
            return Ok(UserInput::Quit);
        }
        let row = row.parse::<i32>()?;
        let col = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        let col = col.parse::<i32>()?;
        return Ok(UserInput::Position(Pos::new(row, col)?));
    }
}
