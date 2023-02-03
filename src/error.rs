//! rsleveldb error module.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RsLevelDBError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    IntParse(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Snappy(#[from] snap::Error),

    #[error("{}", .0)]
    Custom(String),
}
