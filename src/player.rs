pub mod player {
    use crate::{OthelloError, Player, Pos};
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
        fn get_move(&self) -> Result<Pos, OthelloError> {
            use std::io;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let mut iter = input.split(",");
            let x = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
            let x = x.parse::<usize>()?;
            let y = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
            let y = y.parse::<usize>()?;
            return Ok(Pos { x, y });
        }
    }
}
