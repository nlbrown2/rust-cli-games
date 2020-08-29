use crate::error::OthelloError;
use super::{Serialize, Deserialize, RpcError};
const INVALID_ARGS_CODE : u8 = 1;
const FMT_ERROR_CODE : u8 = 2;
const ILLEGAL_MOVE_CODE : u8 = 3;
const IO_ERROR_CODE : u8 = 4;
const PARSE_ERROR_CODE : u8 = 5;

impl Serialize for OthelloError {
    fn serialize(&self) -> Result<Vec<u8>, RpcError> {
        match self {
            OthelloError::InvalidArgs => Ok(vec![INVALID_ARGS_CODE]),
            OthelloError::Fmt(_) => Ok(vec![FMT_ERROR_CODE]),
            OthelloError::IllegalMove => Ok(vec![ILLEGAL_MOVE_CODE]),
            OthelloError::IOError(_) => Ok(vec![IO_ERROR_CODE]),
            OthelloError::ParseError(_) => Ok(vec![PARSE_ERROR_CODE]),
            OthelloError::RemoteError(s) => panic!("Attempting to serialize a remote error with msg: {0}", s),
        }
    }
}

impl Deserialize for OthelloError {
    fn deserialize<'a, T>(buffer: &mut T) -> Result<Self, RpcError>
        where Self: std::marker::Sized,
              T: Iterator<Item=&'a u8>
    {
        let code = *buffer
            .next()
            .ok_or(RpcError::DeserializeError)?;
        match code {
            INVALID_ARGS_CODE => Ok(OthelloError::RemoteError(String::from("Invalid Args"))),
            FMT_ERROR_CODE => Ok(OthelloError::RemoteError(String::from("Format Error"))),
            ILLEGAL_MOVE_CODE => Ok(OthelloError::RemoteError(String::from("Illegal move"))),
            IO_ERROR_CODE => Ok(OthelloError::RemoteError(String::from("IO Error"))),
            PARSE_ERROR_CODE => Ok(OthelloError::RemoteError(String::from("Parse error"))),
            _ => {
                use std::io::Write;
                std::io::stderr().write_all(b"Unexpected error code for OthelloError deserialization").unwrap();
                Err(RpcError::DeserializeError)
            }
        }

    }
}
