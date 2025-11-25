use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorCondition {
    #[error("Serialization Error: {0}")]
    SerializationErr(String),

    #[error("Deserialization Error: {0}")]
    DeserializationErr(String),
}
