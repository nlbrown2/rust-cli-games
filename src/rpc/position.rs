use super::{Deserialize, RpcError, Serialize};
use crate::Pos;
use std::convert::TryInto;
impl Serialize for Pos {
    fn serialize(&self) -> Result<Vec<u8>, RpcError> {
        let mut result = self.row.to_be_bytes().to_vec();
        let col = self.col.to_be_bytes();
        result.append(&mut col.to_vec());
        Ok(result)
    }
}

impl Deserialize for Pos {
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, RpcError> {
        let row = Pos::read_be_usize(buffer)?;
        let col = Pos::read_be_usize(buffer)?;
        Ok(Pos { row, col })
    }
}
impl Pos {
    fn read_be_usize(input: &mut &[u8]) -> Result<usize, RpcError> {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<usize>());
        *input = rest;
        int_bytes
            .try_into()
            .and_then(|arr| Ok(usize::from_be_bytes(arr)))
            .or(Err(RpcError::DeserializeError))
    }
}
