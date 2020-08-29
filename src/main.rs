use rust_othello::{Game, OthelloError, Player, OthelloGame, HumanPlayer, RemotePlayerClient, Client, Server};
use std::collections::VecDeque;
use std::net::TcpStream;

const DEFAULT_ADDRESS :&str = "localhost:8080";

fn main() -> Result<(), OthelloError> {
    assert_eq!(std::mem::size_of::<usize>(), 8);
    println!("Do you want to set up/connect to a remote game? [Y/n]");
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let local_player = Box::new(HumanPlayer::new());
    if input.trim() == "y" || input.trim() == "yes" {
        if let Ok(stream) = TcpStream::connect(DEFAULT_ADDRESS) {
            println!("Connected to the server!");
            let mut client = RemotePlayerClient::new(stream, local_player, false);
            client.run()?;
        } else {
            println!("Couldn't connect to server...");
            <rust_othello::player::RemotePlayerClient as Server>::run(local_player)?;
        }
        Ok(())
    } else {
        let mut players : VecDeque<Box<dyn Player>> = VecDeque::new();
        players.push_back(local_player);
        players.push_back(Box::new(HumanPlayer::new()));
        let mut g = OthelloGame::new(players)?;
        g.run()?;
        Ok(())
    }
}
