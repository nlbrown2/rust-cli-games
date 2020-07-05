use crate::{Game, OthelloError, Player, Pos, UserInput};
use std::fmt;
use std::collections::VecDeque;

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

const BOARD_WIDTH: usize = 8;
#[derive(Debug)]
pub struct GameBoard {
    board: [[char; BOARD_WIDTH]; BOARD_WIDTH],
}

impl Game for OthelloGame {
    fn new(mut players: VecDeque<Box<dyn Player>>) -> Result<OthelloGame, OthelloError> {
        let player1 = players.pop_front().ok_or(OthelloError::InvalidArgs)?;
        let player2 = players.pop_front().ok_or(OthelloError::InvalidArgs)?;
        if !players.is_empty() {
            Err(OthelloError::InvalidArgs)
        } else {
            Ok(OthelloGame {
                player1: player1,
                player2: player2,
                player1_turn: true,
                board: GameBoard::new(),
            })
        }
    }
    fn run(&mut self) -> Result<(), OthelloError> {
        loop {
            println!("The game looks like: {}", &self);
            println!("Please type where you want to move in X, Y format");
            let moved = self.move_next_player()?;
            println!("The game looks like: {}", &self);
            println!("Please type where you want to move in X, Y format");
            let moved = self.move_next_player()? || moved;
            if !moved {
                break;
            }
        }
        Ok(())
    }
}

impl OthelloGame {
    fn move_next_player(&mut self) -> Result<bool, OthelloError> {
        // First, determine if we can move:
        if self.player1_turn && !self.board.player1_can_move() {
            return Ok(false);
        } else if !self.player1_turn && !self.board.player2_can_move() {
            return Ok(false);
        }

        // get the player's move
        let player = if self.player1_turn {
            &self.player1
        } else {
            &self.player2
        };
        let input = player.get_move()?;

        //see if they put in a desire to end the game or not
        let mut moved = false;
        match input {
            UserInput::Position(p) => {
                // they entered a desired position to move to
                if self.player1_turn {
                    self.board.move_player1(&p)?;
                } else {
                    self.board.move_player2(&p)?;
                }
                moved = true;
            },
            UserInput::Quit => {
                println!("Quitting detected!");
                //TODO: actually end the game...Probably replace the "Bool" with an enum, or add a "Quitting" state to the errors (even though it isn't really an error....)
            }
        }

        //flip whose turn it is
        self.player1_turn = !self.player1_turn;
        Ok(moved)
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?; // get onto our own line, screw anyone who tries telling us differently!
        for val in 1..=8 {
            write!(f, "{0: >4}", val)?; // write the column numbers, 4 spaces per column
        }
        write!(f, "\n{}\n", format!("  {0:—^32}", "—"))?; // write 32 '—' characters. 32 = 8 (width in columns) * 4 (width of column)
        for (idx, row) in self.board.iter().enumerate() {
            write!(f, "{}", format!("{0: <1} |", idx + 1))?; // write the 1 digit row number, then a | symbol
            for c in row {
                write!(f, "{0: ^3}|", c)? // write each character, then a pipe symbol. Using 3 spaces ensures the cell's value is centered
            }
            write!(f, "\n{}\n", format!("  {0:—^32}", "—"))?; // write another 32 dashes
        }
        Ok(())
    }
}

const P1_TOKEN: char = '*';
const P2_TOKEN: char = 'O';
const EMPTY_TOKEN: char = ' ';
impl GameBoard {
    fn new() -> GameBoard {
        return GameBoard {board: [[EMPTY_TOKEN; BOARD_WIDTH]; BOARD_WIDTH]};
    }
    fn move_player1(&mut self, pos: &Pos) -> Result<(), OthelloError> {
        self.make_move(&pos, P1_TOKEN)
    }
    fn move_player2(&mut self, pos: &Pos) -> Result<(), OthelloError> {
        self.make_move(&pos, P2_TOKEN)
    }
    fn player1_can_move(&self) -> bool {
        self.can_move(P1_TOKEN)
    }
    fn player2_can_move(&self) -> bool {
        self.can_move(P2_TOKEN)
    }
    fn can_move(&self, _token: char) -> bool {
        true //TODO: check if a given token can be placed anywhere
    }
    fn make_move(&mut self, pos: &Pos, token: char) -> Result<(), OthelloError> {
        if pos.x > BOARD_WIDTH || pos.y > BOARD_WIDTH || pos.x == 0 || pos.y == 0 {
            return Err(OthelloError::IllegalMove);
        }
        let pos = Pos {x : pos.x -1, y: pos.y - 1}; // shift indicies from human-like 1 indexing to computer-like 0 indexing
        if self.board[pos.x][pos.y] != EMPTY_TOKEN { // don't allow players to occupy a taken spot!
            Err(OthelloError::IllegalMove)
        } else {
            self.board[pos.x][pos.y] = token;
            Ok(())
        }
    }
}
