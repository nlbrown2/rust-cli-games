/*!!
 * Implementation of serialize and deserialize for the Result type
 */
use super::{Deserialize, RpcError, Serialize};
impl<U, E> Serialize for Result<U, E>
where
    U: Serialize,
    E: Serialize,
{
    fn serialize(&self) -> Result<Vec<u8>, RpcError> {
        let my_ref = self.as_ref();
        let ok_flag = if my_ref.is_ok() { 0 } else { 1 };
        let mut buffer: Vec<u8> = vec![ok_flag];
        let to_serialize = if my_ref.is_ok() {
            my_ref.ok().unwrap() as &dyn Serialize
        } else {
            my_ref.err().unwrap() as &dyn Serialize
        };
        buffer.append(&mut to_serialize.serialize()?);
        Ok(buffer)
    }
}

impl<U, E> Deserialize for Result<U, E>
where
    U: Deserialize,
    E: Deserialize,
{
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, RpcError> {
        let err_byte = buffer.get(0).ok_or(RpcError::DeserializeError)?;
        let mut rest = &buffer[1..];
        let deserialized_result = if *err_byte == 0 {
            Ok(U::deserialize(&mut rest)?)
        } else {
            Err(E::deserialize(&mut rest)?)
        };
        Ok(deserialized_result)
    }
}
