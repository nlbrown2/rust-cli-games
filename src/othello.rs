use crate::{board::GameBoard, Game, OthelloError, Player, UserInput};
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct OthelloGame {
    player1: Box<dyn Player>,
    player2: Box<dyn Player>,
    player1_turn: bool,
    board: GameBoard,
}

impl fmt::Display for OthelloGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Moved,
    Quitting,
    NoMoveOption,
}

impl Game for OthelloGame {
    fn new(mut players: VecDeque<Box<dyn Player>>) -> Result<OthelloGame, OthelloError> {
        let player1 = players.pop_front().ok_or(OthelloError::InvalidArgs)?;
        let player2 = players.pop_front().ok_or(OthelloError::InvalidArgs)?;
        if !players.is_empty() {
            Err(OthelloError::InvalidArgs)
        } else {
            Ok(OthelloGame {
                player1,
                player2,
                player1_turn: true,
                board: GameBoard::new(),
            })
        }
    }
    fn run(&mut self) -> Result<(), OthelloError> {
        loop {
            let moved_enum = self.move_next_player()?;
            if let Move::Quitting = moved_enum {
                break;
            }
            let moved = moved_enum == Move::Moved;
            println!("The game looks like: {}", &self);
            println!("Please type where you want to move in X, Y format, or type \"quit\" to quit the game"); // TODO: Better code re-use. Probably fold into "move_next_player"
            let moved_enum = self.move_next_player()?;
            if let Move::Quitting = moved_enum {
                break;
            }
            let moved = moved_enum == Move::Moved || moved;
            if !moved {
                break;
            }
        }
        Ok(())
    }
}

impl OthelloGame {
    fn move_next_player(&mut self) -> Result<Move, OthelloError> {
        // First, determine if we can move:
        if self.player1_turn && !self.board.player1_can_move() {
            return Ok(Move::NoMoveOption);
        } else if !self.player1_turn && !self.board.player2_can_move() {
            return Ok(Move::NoMoveOption);
        }

        println!("The game looks like: {}", &self);
        loop {
            let result = self.attempt_make_move();
            match result {
                Err(OthelloError::IllegalMove) | Err(OthelloError::ParseError(_)) => {
                    println!("{:}", result.err().unwrap());
                    continue;
                }
                Err(err) => return Err(err), // other error, so propogate it
                Ok(Move::Quitting) => return Ok(Move::Quitting), // Propogate quitting
                Ok(_) => break,              // no errors, so continue onwards
            }
        }

        //flip whose turn it is
        self.player1_turn = !self.player1_turn;
        Ok(Move::Moved)
    }
    fn attempt_make_move(&mut self) -> Result<Move, OthelloError> {
        // get the player's move
        let player = if self.player1_turn {
            &self.player1
        } else {
            &self.player2
        };
        println!("Please type where you want to move in row, column format, or type \"quit\" to quit the game");
        let input = player.get_move()?;

        //see if they put in a desire to end the game or not
        match input {
            UserInput::Position(p) => {
                // they entered a desired position to move to
                if self.player1_turn {
                    self.board.move_player1(&p)?;
                } else {
                    self.board.move_player2(&p)?;
                };
                Ok(Move::Moved)
            }
            // They typed quit
            UserInput::Quit => {
                println!("Quitting detected!");
                return Ok(Move::Quitting);
            }
        }
    }
}
