use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum ErrorKind {
    #[error("Time out")]
    IoTimeOut,
    #[error("Io error")]
    IoError,
    #[error("Deserialization error")]
    DeserializationError,
    #[error("Serialization error")]
    SerializationError,
    #[error("Unknown type packet")]
    UnknownTypePack,
    #[error("Try connect to closed connection")]
    NotOpenedConnection,
}
