enum RpcError {
    SerializeError,
    DeserializeError,
    ConnectionError,
}
trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, RpcError>;
}

trait Deserialize {
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, RpcError>
    where
        Self: std::marker::Sized;
}
pub mod position;
pub mod result;
pub mod user_input;
