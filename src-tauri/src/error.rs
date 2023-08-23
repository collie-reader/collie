#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid value `{0}`")]
    InvalidValue(String),

    #[error("unknown")]
    Unknown,

    #[error(transparent)]
    RusqliteError {
        #[from]
        source: rusqlite::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
