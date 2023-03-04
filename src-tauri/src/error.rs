use biliup::error::Kind;
use futures::future::Aborted;
use std::num::ParseIntError;
use thiserror::Error;
// use anyhow::Result;
use serde::{Serialize, Serializer};

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Err(String),
    #[error(transparent)]
    Error(#[from] Kind),
    #[error(transparent)]
    Aborted(#[from] Aborted),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ReqIO(#[from] reqwest::Error),
    #[error(transparent)]
    YamlIO(#[from] serde_yaml::Error),
    #[error(transparent)]
    JsonIO(#[from] serde_json::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SendError(#[from] tokio::sync::mpsc::error::SendError<u64>),
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

// #[derive(Error, Debug, Serialize)]
// pub enum MyError {
//   #[error(transparent)]
//   Other(#[from] #[serde(skip)] reqwest::Error),  // source and Display delegate to anyhow::Error
// }
pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Err(s)
    }
}
