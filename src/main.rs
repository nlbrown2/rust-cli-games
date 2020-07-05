use rust_othello::othello::OthelloGame;
use rust_othello::player::player;
use rust_othello::{Game, OthelloError, Player};

fn main() -> Result<(), OthelloError> {
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(player::HumanPlayer::new()),
        Box::new(player::HumanPlayer::new()),
    ];
    let mut g = OthelloGame::new(players)?;
    g.run()?;
    Ok(())
}
