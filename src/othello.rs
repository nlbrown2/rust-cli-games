use crate::{Game, OthelloError, Player, Pos};
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

#[derive(Debug)]
pub struct GameBoard {
    board: Vec<Vec<char>>,
}

impl Game for OthelloGame {
    fn new(mut players: Vec<Box<dyn Player>>) -> Result<OthelloGame, OthelloError> {
        if players.len() != 2 {
            return Err(OthelloError::InvalidArgs);
        }
        Ok(OthelloGame {
            player1: players.remove(0),
            player2: players.remove(0),
            player1_turn: true,
            board: GameBoard::new(),
        })
    }
    fn run(&mut self) -> Result<(), OthelloError> {
        let mut moved = true;
        while moved {
            println!("The game looks like: {}", &self);
            println!("Please type where you want to move in X, Y format");
            moved = false;
            if self.board.player1_can_move() {
                let pos = self.player1.get_move()?;
                self.board.move_player1(&pos)?;
                moved = true;
            }
            println!("The game looks like: {}", &self);
            println!("Please type where you want to move in X, Y format");
            if self.board.player2_can_move() {
                let pos = self.player2.get_move()?;
                self.board.move_player2(&pos)?;
                moved = true;
            }
        }
        Ok(())
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for val in 1..=8 {
            write!(f, "{0: >4}", val)?;
        }
        write!(f, "\n{}\n", format!("  {0:—^32}", "—"))?;
        for (idx, row) in self.board.iter().enumerate() {
            write!(f, "{}", format!("{0: <1} |", idx + 1))?;
            for c in row {
                write!(f, "{0: ^3}|", c)?
            }
            write!(f, "\n{}\n", format!("  {0:—^32}", "—"))?;
        }
        Ok(())
    }
}

const BOARD_WIDTH: usize = 8;
const P1_TOKEN: char = '*';
const P2_TOKEN: char = 'O';
impl GameBoard {
    fn new() -> GameBoard {
        let board = vec![vec![' '; BOARD_WIDTH]; BOARD_WIDTH];
        return GameBoard { board };
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
        true
    }
    fn make_move(&mut self, pos: &Pos, token: char) -> Result<(), OthelloError> {
        if pos.x > BOARD_WIDTH || pos.y > BOARD_WIDTH || pos.x == 0 || pos.y == 0 {
            return Err(OthelloError::IllegalMove);
        }
        self.board[pos.x - 1][pos.y - 1] = token;
        Ok(())
    }
}
