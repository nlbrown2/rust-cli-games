use rust_othello::{Game, OthelloError, Player, OthelloGame, HumanPlayer};

fn main() -> Result<(), OthelloError> {
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(HumanPlayer::new()),
        Box::new(HumanPlayer::new()),
    ];
    let mut g = OthelloGame::new(players)?;
    g.run()?;
    Ok(())
}
