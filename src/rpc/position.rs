use super::{Deserialize, RpcError, Serialize};
use crate::Pos;
impl Serialize for Pos {
    fn serialize(&self) -> Result<Vec<u8>, RpcError> {
        let mut result = self.row.to_be_bytes().to_vec();
        let col = self.col.to_be_bytes();
        result.append(&mut col.to_vec());
        Ok(result)
    }
}

impl Deserialize for Pos {
    fn deserialize<'a, T>(buffer: &mut T) -> Result<Self, RpcError>
    where T: Iterator<Item=&'a u8>
    {
        let row = Pos::read_be_usize(buffer)?;
        let col = Pos::read_be_usize(buffer)?;
        Ok(Pos { row, col })
    }
}
impl Pos {
     fn read_be_usize<'a, T>(input: &mut T) -> Result<usize, RpcError>
     where T: Iterator<Item=&'a u8>
     {
        let mut usize_buffer : [u8; 8] = [0; 8];
        println!("Attempting to read 8 bytes...");
        for i in 0..8 {
            let val : u8 = *input
                .next()
                .ok_or(RpcError::DeserializeError)?;
            usize_buffer[i] = val;
        }
        println!("Read 8 bytes!");
        Ok(usize::from_be_bytes(usize_buffer))
    }
}
