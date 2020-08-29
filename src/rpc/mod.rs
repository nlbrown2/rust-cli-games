use crate::OthelloError;
use std::io;
pub enum RpcError {
    DeserializeError,
    IOError(io::Error),
}
pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, RpcError>;
}

pub trait Deserialize {
    fn deserialize<'a, T>(buffer: &mut T) -> Result<Self, RpcError>
    where
        Self: std::marker::Sized,
        T: Iterator<Item=&'a u8>;
}

impl From<RpcError> for OthelloError {
    fn from(error: RpcError) -> OthelloError {
        match error {
            RpcError::DeserializeError => OthelloError::RemoteError(String::from("Deserialization error in RPC call!")),
            RpcError::IOError(err) => OthelloError::IOError(err),
        }
    }
}

impl From<io::Error> for RpcError {
    fn from(error: io::Error) -> RpcError {
        RpcError::IOError(error)
    }
}
pub mod position;
pub mod result;
pub mod user_input;
pub mod othello_error;
