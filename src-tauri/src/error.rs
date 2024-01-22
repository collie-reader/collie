use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid value `{0}`")]
    InvalidValue(String),

    #[error("invalid key `{0}` for `{1}`")]
    InvalidEnumKey(String, String),

    #[error("invalid feed link `{0}`")]
    InvalidFeedLink(String),

    #[error("forbidden")]
    Forbidden,

    #[error("failed to parse syndication feed")]
    SyndicationParsingFailure,

    #[error("empty string")]
    EmptyString,

    #[error("unknown")]
    Unknown,

    #[error(transparent)]
    RusqliteError {
        #[from]
        source: rusqlite::Error,
    },

    #[error(transparent)]
    SeaQueryError {
        #[from]
        source: sea_query::error::Error,
    },

    #[error(transparent)]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error(transparent)]
    IoError {
        #[from]
        source: io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
