use mlua::Error as LuaError;
use starknet::accounts::single_owner::SignError as AccountSignError;
use starknet::accounts::AccountError;
use starknet::core::types::contract::JsonError;
use starknet::core::types::FromStrError;
use starknet::core::utils::NonAsciiNameError;
use starknet::providers::ProviderError;
use starknet::signers::local_wallet::SignError;
use std::fmt;

use thiserror::Error;

/// Result type used for Kipt error management.
pub type KiptResult<T, E = Error> = Result<T, E>;

/// Kipt errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occured: {0}")]
    Other(String),
    #[error(transparent)]
    StarknetProvider(#[from] ProviderError),
    #[error(transparent)]
    StarknetFromStr(#[from] FromStrError),
    #[error(transparent)]
    StdIO(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    ContractJson(#[from] JsonError),
    #[error(transparent)]
    AccountError(#[from] AccountError<AccountSignError<SignError>>),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Contract artifacts is missing: {0}")]
    ArtifactsMissing(String),
    #[error(transparent)]
    NonAsciiNameError(#[from] NonAsciiNameError),
}

impl From<Error> for LuaError {
    fn from(e: Error) -> Self {
        Self::ExternalError(std::sync::Arc::new(ErrorExtLua::new(&e.to_string())))
    }
}

// TODO: check if we can implement error for lua with enum!

/// This error type is mainly used to interact with mlua library,
/// which is expecting a struct.
#[derive(Debug, Clone)]
pub struct ErrorExtLua {
    reason: String,
}

impl ErrorExtLua {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for ErrorExtLua {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for ErrorExtLua {}
