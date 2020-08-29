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
    fn deserialize<'a, T>(buffer: &mut T) -> Result<Self, RpcError>
    where T: Iterator<Item=&'a u8>
    {
        println!("Getting error byte!");
        let err_byte = buffer.next().ok_or(RpcError::DeserializeError)?;
        let deserialized_result = if *err_byte == 0 {
            Ok(U::deserialize(buffer)?)
        } else {
            println!("Could not deserialize result!");
            Err(E::deserialize(buffer)?)
        };
        Ok(deserialized_result)
    }
}
