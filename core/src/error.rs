#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Json error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("FromStrRadixError: {0}")]
    FromStrRadixError(#[from] ethers::abi::ethereum_types::FromStrRadixErr),
    #[error("BoundsError")]
    BoundsError,
    #[error("DecodeError")]
    DecodeError,
    #[error("Error: {0}")]
    GenericError(String),
}
