use crate::slip10;
use bip39;
use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidDifficulty,
    EmptyTxId,
    InvalidBlockHash,
    Bad,
    PrivateKeyCredentialUnsupported,
    InvalidIndex,
    InvalidSalt,
    InvalidHexCypherText,
    Slip10Error(slip10::Error),
    Bip39Error(bip39::Error),
    IoError(io::Error),
    GrpcTransportError(tonic::transport::Error),
    GrpcError(tonic::Status),
    HexEncodingError(hex::FromHexError),
    Ed25519Error(ed25519_compact::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "keystore error: {}", self.desc())
    }
}

impl From<ed25519_compact::Error> for Error {
    fn from(error: ed25519_compact::Error) -> Self {
        Error::Ed25519Error(error)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(error: hex::FromHexError) -> Self {
        Error::HexEncodingError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Self {
        Error::GrpcTransportError(error)
    }
}

impl From<tonic::Status> for Error {
    fn from(error: tonic::Status) -> Self {
        Error::GrpcError(error)
    }
}

impl From<bip39::Error> for Error {
    fn from(error: bip39::Error) -> Self {
        Error::Bip39Error(error)
    }
}

impl From<slip10::Error> for Error {
    fn from(error: slip10::Error) -> Self {
        Error::Slip10Error(error)
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            PrivateKeyCredentialUnsupported => "raw private keys are not supported".into(),
            Bad => "you're a bad boi!!!".into(),
            InvalidIndex => "index must be > 0".into(),
            InvalidSalt => "salt must be in valid hex format".into(),
            InvalidHexCypherText => "cypher_test must be in valid hex format".into(),
            Slip10Error(e) => format!("slip10 error: {}", e.desc()),
            Bip39Error(e) => format!("bip39 error: {}", e),
            IoError(e) => format!("IO error: {}", e),
            GrpcTransportError(e) => format!("GRPC transport error: {}", e),
            GrpcError(e) => format!("GRPC error: {}", e),
            HexEncodingError(e) => format!("hex encoding error: {}", e),
            Ed25519Error(e) => format!("ed25519 error: {}", e),
            InvalidDifficulty => "invalid difficulty".into(),
            EmptyTxId => "empty transaction id".into(),
            InvalidBlockHash => "invalid block hash".into(),
        }
    }
}
