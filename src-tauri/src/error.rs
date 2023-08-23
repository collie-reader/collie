#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid value `{0}`")]
    InvalidValue(String),

    #[error("invalid key `{0}` for `{1}`")]
    InvalidEnumKey(String, String),

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
}

pub type Result<T> = std::result::Result<T, Error>;
