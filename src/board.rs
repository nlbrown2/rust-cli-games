use crate::{OthelloError, Pos};
use std::fmt;
pub const BOARD_WIDTH: usize = 8;
#[derive(Debug)]
pub struct GameBoard {
    board: [[char; BOARD_WIDTH]; BOARD_WIDTH],
}
impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?; // get onto our own line, screw anyone who tries telling us differently!
        for val in 1..=BOARD_WIDTH {
            write!(f, "{0: >4}", val)?; // write the column numbers, 4 spaces per column
        }
        write!(f, "\n{}\n", format!("  {0:—^32}", "—"))?; // write 32 '—' characters. 32 = 8 (width in columns) * 4 (width of column)
        for row in 0..BOARD_WIDTH {
            write!(f, "{}", format!("{0: <1} |", row + 1))?; // write the 1 digit row number, then a | symbol
            for col in 0..BOARD_WIDTH {
                write!(f, "{0: ^3}|", self.board[row][col])? // write each character, then a pipe symbol. Using 3 spaces ensures the cell's value is centered
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
    pub fn new() -> GameBoard {
        let mut empty = GameBoard {
            board: [[EMPTY_TOKEN; BOARD_WIDTH]; BOARD_WIDTH],
        };
        empty.board[BOARD_WIDTH / 2][BOARD_WIDTH / 2] = P1_TOKEN;
        empty.board[BOARD_WIDTH / 2][BOARD_WIDTH / 2 - 1] = P2_TOKEN;
        empty.board[BOARD_WIDTH / 2 - 1][BOARD_WIDTH / 2] = P2_TOKEN;
        empty.board[BOARD_WIDTH / 2 - 1][BOARD_WIDTH / 2 - 1] = P1_TOKEN;
        empty
    }
    pub fn move_player1(&mut self, pos: &Pos) -> Result<(), OthelloError> {
        self.make_move(&pos, P1_TOKEN)
    }
    pub fn move_player2(&mut self, pos: &Pos) -> Result<(), OthelloError> {
        self.make_move(&pos, P2_TOKEN)
    }
    pub fn player1_can_move(&self) -> bool {
        self.can_move(P1_TOKEN)
    }
    pub fn player2_can_move(&self) -> bool {
        self.can_move(P2_TOKEN)
    }
    fn can_move(&self, token: char) -> bool {
        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        if self
                            .determine_end_of_run(&Pos { row, col }, dr, dc, token)
                            .is_some()
                        {
                            println!(
                                "Can go to: {:?} with dr={:}, dc={:}",
                                Pos { row, col },
                                dr,
                                dc
                            );
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn make_move(&mut self, pos: &Pos, token: char) -> Result<(), OthelloError> {
        if pos.row > BOARD_WIDTH || pos.col > BOARD_WIDTH || pos.row == 0 || pos.col == 0 {
            return Err(OthelloError::IllegalMove);
        }
        let pos = Pos {
            row: pos.row - 1,
            col: pos.col - 1,
        }; // shift indicies from human-like 1 indexing to computer-like 0 indexing
        if self.board[pos.row][pos.col] != EMPTY_TOKEN {
            // don't allow players to occupy a taken spot!
            Err(OthelloError::IllegalMove)
        } else {
            self.board[pos.row][pos.col] = token;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                }
            }
            Ok(())
        }
        // TODO: flip opponents token
        // find which directions have my token along them
        // flip all tokens on the way back
        //  - can either remember them or can just iterate forwards then backwards (probs this one)
        // Have a generic function that generates the next row & column given the current row &
        // column. Can do so by pasing in dX & dY. While the next generated row & col is still valid,
        // see if it is the type of token we are looking for, if so, then flip all of the indicies
        // between that and the start (by intervting dx & dy) & return the number of tokens you
        // flipped. That will inform the caller how to update the score.
        // for dx = -1 to 1
        //    for dy = -1 to 1
        //       skip 0, 0
        //       etc.
        // TODO: Track score
    }
    /**
     * Flips all opponent tokens starting at start_pos moving in the direction of dx & dy.
     * TODO: update my doc
     */
    fn flip_files(
        &mut self,
        start_pos: &Pos,
        dr: i32,
        dc: i32,
        token: char,
    ) -> Result<usize, OthelloError> {
        let end_pos = self.determine_end_of_run(start_pos, dr, dc, token);
        end_pos.map(|_| 5).ok_or(OthelloError::IllegalMove)
    }

    /**
     * Starting at start_pos, provides the end position of a run. Returns None if there
     * is no run in the specified direction starting at start_pos
     */
    fn determine_end_of_run(&self, start_pos: &Pos, dr: i32, dc: i32, token: char) -> Option<Pos> {
        let other_token = if token == P1_TOKEN {
            P2_TOKEN
        } else {
            P1_TOKEN
        };
        let mut end_range: Pos = start_pos.clone();
        end_range.shift(dr, dc).ok()?;
        let mut will_flip_a_token = false;
        while self.board[end_range.row][end_range.col] == other_token {
            end_range.shift(dr, dc).ok()?;
            will_flip_a_token = true;
        }
        if !will_flip_a_token || self.board[end_range.row][end_range.col] != token {
            return None;
        }
        Some(end_range)
    }
}
