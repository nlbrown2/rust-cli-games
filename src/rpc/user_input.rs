/*!!
 * Implementation of serialize and deserialize for the user input enum
 */
use super::{Deserialize, RpcError, Serialize};
use crate::{Pos, UserInput};
impl Serialize for UserInput {
    fn serialize(&self) -> Result<Vec<u8>, RpcError> {
        let variant_flag = match &self {
            UserInput::Position(_) => 0,
            UserInput::Quit => 1,
        };
        let mut buffer: Vec<u8> = vec![variant_flag];
        if let UserInput::Position(pos) = &self {
            buffer.append(&mut pos.serialize()?);
        }
        Ok(buffer)
    }
}

impl Deserialize for UserInput {
    fn deserialize<'a, T>(buffer: &mut T) -> Result<Self, RpcError>
    where T: Iterator<Item=&'a u8>
    {
        let variant_flag: u8 = *buffer.next().ok_or(RpcError::DeserializeError)?;
        println!("Variant flag: {0}", variant_flag);
        match variant_flag {
            0 => Ok(UserInput::Position(Pos::deserialize(buffer)?)),
            1 => Ok(UserInput::Quit),
            _ => Err(RpcError::DeserializeError)
        }
    }
}
