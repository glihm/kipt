use anyhow::Result;
use mlua::{Error as LuaError, Lua, Number, Result as LuaResult, Table};
use starknet::{
    core::types::{ExecutionResult, FieldElement, StarknetError},
    providers::{MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage},
};
use std::time::Duration;
use tracing::trace;

use crate::account;
use crate::error::ErrorExtLua;
use crate::lua::{self, LuaOutput, RT};

/// Defines a lua function to watch a transaction from it's hash.
/// Watching is polling the receipt until it's available on-chain.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
/// * `transaction_hash` - The transaction hash to poll receipt for.
/// * `interval_ms` - Interval in milliseconds for the polling.
pub fn lua_watch(
    lua: &'_ Lua,
    transaction_hash: String,
    interval_ms: Number,
) -> LuaResult<Table<'_>> {
    let url_network = lua::get_provider(lua)?;

    let interval_ms = Duration::from_millis(interval_ms as u64);
    let transaction_hash = FieldElement::from_hex_be(&transaction_hash).map_err(|_e| {
        LuaError::ExternalError(std::sync::Arc::new(ErrorExtLua::new(&format!(
            "Invalid FieldElement value: {}",
            transaction_hash
        ))))
    })?;

    let data = futures::executor::block_on(async move {
        RT.spawn(async move {
            let provider = match account::setup_provider(&url_network).await {
                Ok(a) => a,
                Err(e) => {
                    return LuaOutput {
                        data: None,
                        error: format!("{:?}", e),
                    }
                }
            };

            match poll_exec_succeeded(provider, transaction_hash, interval_ms).await {
                Ok(()) => LuaOutput {
                    data: Some(()),
                    error: "".to_string(),
                },
                Err(e) => LuaOutput {
                    data: None,
                    error: format!("{:?}", e),
                },
            }
        })
        .await
        .unwrap()
    });

    if let Some(_d) = data.data {
        let t = lua.create_table()?;

        // No data in this one (for now).

        Ok(t)
    } else {
        Err(LuaError::ExternalError(std::sync::Arc::new(
            ErrorExtLua::new(&data.error),
        )))
    }
}

pub async fn poll_exec_succeeded<P>(
    provider: P,
    transaction_hash: FieldElement,
    poll_interval: Duration,
) -> Result<()>
where
    P: Provider,
{
    loop {
        match provider.get_transaction_receipt(transaction_hash).await {
            Ok(receipt) => match receipt.execution_result() {
                ExecutionResult::Succeeded => {
                    trace!(
                        "Transaction {} confirmed",
                        format!("0x{:064x}", transaction_hash)
                    );

                    return Ok(());
                }
                ExecutionResult::Reverted { reason } => {
                    return Err(anyhow::anyhow!("transaction reverted: {}", reason));
                }
            },
            Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound),
                ..
            })) => {
                trace!("Transaction not confirmed yet...");
            }
            // Some nodes are still serving error code `25` for tx hash not found. This is
            // technically a bug on the node's side, but we maximize compatibility here by also
            // accepting it.
            Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                code: MaybeUnknownErrorCode::Known(StarknetError::InvalidTransactionHash),
                ..
            })) => {
                trace!("Transaction not confirmed yet...");
            }
            Err(err) => return Err(err.into()),
        }

        tokio::time::sleep(poll_interval).await;
    }
}
