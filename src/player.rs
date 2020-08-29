use crate::{OthelloError, Player, Pos, UserInput, Client, Server, OthelloGame, Game};
use std::net::{TcpStream, TcpListener};
use std::collections::VecDeque;
use std::io::Write;
use crate::rpc::{Serialize, Deserialize};
use crate::board::{GameBoard, BOARD_WIDTH};
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
    fn get_move(&self, board: &GameBoard) -> Result<UserInput, OthelloError> {
        use std::io;
        println!("The game looks like: {}", board);
        println!("Please type where you want to move in X, Y format, or type \"quit\" to quit the game");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.split(",");
        let row = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        if row == "q" || row == "quit" {
            // TODO: normalize input so things like Quit or Q also work
            return Ok(UserInput::Quit);
        }
        let row = row.parse::<i32>()?;
        let row = row - 1;
        let col = iter.next().ok_or(OthelloError::InvalidArgs)?.trim();
        let col = col.parse::<i32>()?;
        let col = col - 1;
        let pos = Pos::new(row, col)?;
        return Ok(UserInput::Position(pos));
    }
}

use std::cell::RefCell;

#[derive(Debug)]
pub struct RemotePlayerClient {
    stream : RefCell<TcpStream>,
    local_player: Box<dyn Player>,
    is_server: bool,
}

impl RemotePlayerClient {
    pub fn new(stream: TcpStream, local_player: Box<dyn Player>, is_server: bool) -> Self {
        RemotePlayerClient {
            stream: RefCell::new(stream),
            local_player,
            is_server
        }
    }
}

impl Client for RemotePlayerClient {
    fn run(&mut self) -> Result<(), OthelloError>{
        use std::io::Read;
        let mut buf: [u8; 1] = [0];
        println!("about to read a single byte");
        while self.stream.borrow_mut().read_exact(&mut buf).is_ok() {
            println!("Read: {0}", buf[0]);
            match buf[0] {
                GET_MOVE_CODE => {
                    println!("Getting local move!");
                    let mut buf = [0; BOARD_WIDTH * BOARD_WIDTH];
                    self.stream.borrow_mut().read_exact(&mut buf)?;
                    let board = GameBoard::deserialize(&mut buf.iter())?;
                    //TODO: read & print the game board
                    let result = self.local_player.get_move(&board);
                    let mut buff = result.serialize()?;
                    self.stream.borrow_mut().write_all(&buff.len().to_be_bytes())?;
                    self.stream.borrow_mut().write_all(&mut buff)?;
                }
                _ => panic!("Unexpected code received; Nathan writes bugs")
            }

            // match on first byte, execute RPC call by forwarding args to the local_player
        }
        Ok(())
    }
}

impl Server for RemotePlayerClient {
    fn run(local_player: Box<dyn Player>) -> Result<(), OthelloError> {
        let mut players = VecDeque::new();
        players.push_back(local_player);
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let (stream, _) = listener.accept()?;
        players.push_back(Box::new(RemotePlayerClient::new(stream, Box::new(HumanPlayer::new()), true)));
        let mut g = OthelloGame::new(players)?;
        g.run()?;
        Ok(())
    }
}

const GET_MOVE_CODE : u8 = 0b1;

impl Player for RemotePlayerClient {
    fn new() -> Self {
        unimplemented!();
    }
    fn get_move(&self, board: &GameBoard) -> Result<UserInput, OthelloError> {
        println!("Getting remote user input!");
        if self.is_server {
            use std::io::Read;
            self.stream.borrow_mut().write(&[GET_MOVE_CODE])?;
            self.stream.borrow_mut().write_all(board.serialize()?.as_slice())?;
            // make RPC call
            // TODO: serialize & send over the game board...luckily that shouldn't be too hard....
            // getting a move should depend on the game state anyways...also makes it more
            // testable. So, modify the fn interface to accept a game board!
            let mut buf : [u8; 8] = [0; 8];
            println!("Reading size");
            self.stream.borrow_mut().read_exact(&mut buf)?;
            let num_bytes = usize::from_be_bytes(buf);
            let mut rest_bytes : Vec<u8> = Vec::with_capacity(num_bytes);
            rest_bytes.resize(num_bytes, 0);
            println!("Reading {0} bytes", num_bytes);
            self.stream.borrow_mut().read_exact(&mut rest_bytes)?;
            println!("{:?}", rest_bytes);
            let result : Result<UserInput, OthelloError> = Result::deserialize(&mut rest_bytes.iter())?;
            //TODO: figure out where the deserialization is failing
            result
        } else {
            unimplemented!();
        }
    }
}
