use thiserror::Error;
// use anyhow::Result;
use serde::{Serialize, Deserialize, Serializer};

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Err(String),
    #[error(transparent)]
    ReqIO(#[from] reqwest::Error),
    #[error(transparent)]
    YamlIO(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

// #[derive(Error, Debug, Serialize, Display)]
// pub struct MyError {
//   msg: String,
//   #[source]  // optional if field name is `source`
//   source: anyhow::Error,
// }
// #[derive(Error, Debug, Serialize)]
// #[error("{msg}")]
// pub struct MyError {
//   msg: String,
//   // backtrace: Backtrace,
//   #[serde(skip)]
//   // #[from]  // optional if field name is `source`
//   source: reqwest::Error,
// }
//
// impl From<reqwest::Error> for MyError {
//   fn from(err: reqwest::Error) -> MyError {
//     MyError { msg: err.to_string(), source: err }
//   }
// }
// impl <E> From<E> for Error
//     where
//         E: std::error::Error + Send + Sync + 'static,
// {
//     fn from(err: E) -> Error {
//         Error::Other(anyhow::Error::from(err))
//     }
// }

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        match self {
            Error::IO(io) => serializer.serialize_str(&io.to_string()),
            Error::Other(other) => serializer.serialize_str(&other.to_string()),
            Error::ReqIO(e) => serializer.serialize_str(&e.to_string()),
            Error::YamlIO(e) => serializer.serialize_str(&e.to_string()),
            Error::Err(e) => serializer.serialize_str(&e.to_string())
        }
    }
}


// #[derive(Error, Debug, Serialize)]
// pub enum MyError {
//   #[error(transparent)]
//   Other(#[from] #[serde(skip)] reqwest::Error),  // source and Display delegate to anyhow::Error
// }
pub type Result<T, E=Error> = core::result::Result<T, E>;
