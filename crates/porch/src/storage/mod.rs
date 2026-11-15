use thiserror::Error;

pub mod memory;

#[derive(Debug, Error)]
pub enum StorageError<E = !> {
    #[error("target resource does not exist")]
    NoExist,
    #[error(transparent)]
    Specific(#[from] E),
    #[error(transparent)]
    Other(anyhow::Error),
}

pub type StorageResult<T, E = !> = Result<T, StorageError<E>>;

pub trait Storage: Send + Sync {}
