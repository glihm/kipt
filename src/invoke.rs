use mlua::{Error as LuaError, FromLua, Lua, Result as LuaResult, Table, Value};
use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    core::types::{FieldElement, InvokeTransactionResult},
    core::utils::get_selector_from_name,
    providers::AnyProvider,
    signers::LocalWallet,
};

use std::time::Duration;

use crate::error::{ErrorExtLua, KiptResult};
use crate::lua::{self, LuaOutput, LuaTableSetable, RT};
use crate::{account, logger, transaction};

/// Invoke call.
/// TODO: implement the FromLua trait.
pub struct InvokeCall {
    pub to: String,
    pub func: String,
    pub calldata: Vec<String>,
}

impl<'lua> FromLua<'lua> for InvokeCall {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(t) = value {
            Ok(InvokeCall {
                to: t.get("to")?,
                func: t.get("func")?,
                calldata: t.get("calldata")?,
            })
        } else {
            Err(LuaError::ExternalError(std::sync::Arc::new(
                ErrorExtLua::new(&format!(
                    "Can't convert the value {:?} into InvokeCall",
                    value
                )),
            )))
        }
    }
}

/// Invoke output.
struct InvokeOutput {
    pub transaction_hash: String,
}

impl LuaTableSetable for InvokeOutput {
    fn set_all(&self, table: &Table) {
        table.set("tx_hash", self.transaction_hash.clone()).unwrap();
    }
}

/// Defines a lua function that sends an invoke transaction to a contract.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
/// * `calls` - Calls to be added in the invoke transaction.
/// * `options` - Options for the invoke transaction.
pub fn lua_invoke<'lua>(
    lua: &'lua Lua,
    calls: Vec<InvokeCall>,
    options: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    let (url_network, address, privkey, is_legacy) = lua::get_account(lua)?;

    let watch_interval = lua::get_watch_from_options(&options)?;

    let mut out_log = String::from(&format!("> invoke: ({})\\n", calls.len()));
    for (i, c) in calls.iter().enumerate() {
        out_log.push_str(&format!("call #{} -> {} {}\\n", i, c.to, c.func));
    }

    let data = futures::executor::block_on(async move {
        RT.spawn(async move {
            let account =
                match account::setup_account(&url_network, &address, &privkey, is_legacy).await {
                    Ok(a) => a,
                    Err(e) => {
                        return LuaOutput {
                            data: None,
                            error: format!("{:?}", e),
                        }
                    }
                };

            match invoke_tx(account, calls, watch_interval).await {
                Ok(invk_res) => LuaOutput {
                    data: Some(InvokeOutput {
                        transaction_hash: format!("0x{:064x}", invk_res.transaction_hash),
                    }),
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

    if let Some(d) = data.data {
        let t = lua.create_table()?;
        d.set_all(&t);

        out_log.push_str(&format!(
            "|     tx_hash      |  {}  |\\n",
            d.transaction_hash
        ));
        logger::write(lua, &out_log)?;

        Ok(t)
    } else {
        out_log.push_str(&format!("error: {}\\n", data.error));

        Err(LuaError::ExternalError(std::sync::Arc::new(
            ErrorExtLua::new(&data.error),
        )))
    }
}

/// Sends an invoke transaction to a contract.
///
/// # Arguments
///
/// * `account` - The account used to sign and send the transaction.
/// * `calls` - The list of calls to be executed.
/// * `watch_interval` - Watch interval for the transaction receipt.
async fn invoke_tx(
    account: SingleOwnerAccount<AnyProvider, LocalWallet>,
    calls: Vec<InvokeCall>,
    watch_interval: Option<Duration>,
) -> KiptResult<InvokeTransactionResult> {
    // TODO: add fee estimate.

    let mut sn_calls = vec![];

    for c in calls {
        let to = FieldElement::from_hex_be(&c.to)?;
        let selector = get_selector_from_name(&c.func)?;

        let mut calldata = vec![];
        for cd in c.calldata {
            calldata.push(FieldElement::from_hex_be(&cd)?);
        }

        sn_calls.push(Call {
            to,
            selector,
            calldata,
        });
    }

    let invk_res = account.execute(sn_calls).send().await?;

    if let Some(interval) = watch_interval {
        transaction::poll_exec_succeeded(account.provider(), invk_res.transaction_hash, interval)
            .await?;
    }

    Ok(invk_res)
}
