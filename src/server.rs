/*
 * Othello server, runs the same main game loop and uses a remote player to use RPCs to client.
 * Mainly to decouple client from game loop logic- it can just be an RPC shell
 */

use crate::{ Player, OthelloError};
pub trait Server {
    fn run(players: Box<dyn Player>) -> Result<(), OthelloError>;
}
