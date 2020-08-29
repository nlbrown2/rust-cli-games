use crate::OthelloError;
pub trait Client {
    fn run(&mut self) -> Result<(), OthelloError>;
}
