use mlua::{Error as LuaError, Lua, Result as LuaResult, Table};
use starknet::{
    accounts::{Account, ConnectedAccount, SingleOwnerAccount},
    core::types::{
        contract::{CompiledClass, SierraClass},
        DeclareTransactionResult, FieldElement,
    },
    providers::AnyProvider,
    signers::LocalWallet,
};
use std::sync::Arc;
use std::time::Duration;

use crate::error::{ErrorExtLua, KiptResult};
use crate::lua::{self, LuaOutput, LuaTableSetable, RT};
use crate::{account, utils};

/// Declare output.
struct DeclareOutput {
    pub transaction_hash: String,
    pub sierra_class_hash: String,
}

impl LuaTableSetable for DeclareOutput {
    fn set_all(&self, table: &Table) {
        table.set("tx_hash", self.transaction_hash.clone()).unwrap();

        table
            .set("class_hash", self.sierra_class_hash.clone())
            .unwrap();
    }
}

/// Defines a lua function that declares a contract.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
/// * `sierra_path` - Path to Sierra contract class file.
/// * `casm_path` - Path to Casm (compiled) contract class file.
/// * `options` - Options for the declare transaction.
pub fn lua_declare<'lua>(
    lua: &'lua Lua,
    sierra_path: String,
    casm_path: String,
    options: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    let (url_network, address, privkey) = lua::get_account(lua)?;

    let watch_interval = lua::get_watch_from_options(&options)?;

    let data = futures::executor::block_on(async move {
        RT.spawn(async move {
            let account = match account::setup_account(&url_network, &address, &privkey).await {
                Ok(a) => a,
                Err(e) => {
                    return LuaOutput {
                        is_success: false,
                        data: None,
                        error: format!("{:?}", e),
                    }
                }
            };

            match declare_tx(account, &sierra_path, &casm_path, watch_interval).await {
                Ok((class_hash, decl_res)) => LuaOutput {
                    is_success: false,
                    data: Some(DeclareOutput {
                        transaction_hash: format!("0x{:064x}", decl_res.transaction_hash),
                        sierra_class_hash: format!("0x{:064x}", class_hash),
                    }),
                    error: "".to_string(),
                },
                Err(e) => LuaOutput {
                    is_success: false,
                    data: None,
                    error: format!("{:?}", e),
                },
            }
        })
        .await
        .unwrap()
    });

    if data.error.is_empty() {
        let t = lua.create_table()?;

        if let Some(d) = data.data {
            d.set_all(&t);
        }

        Ok(t)
    } else {
        Err(LuaError::ExternalError(std::sync::Arc::new(
            ErrorExtLua::new(&data.error),
        )))
    }
}

/// Sends a transaction to declare a contract.
///
/// # Arguments
///
/// * `account` - The account used to sign and send the transaction.
/// * `sierra_path` - Path to Sierra contract class file.
/// * `casm_path` - Path to Casm (compiled) contract class file.
/// * `watch_interval` - Watch interval for the transaction receipt.
async fn declare_tx(
    account: SingleOwnerAccount<AnyProvider, LocalWallet>,
    sierra_path: &str,
    casm_path: &str,
    watch_interval: Option<Duration>,
) -> KiptResult<(FieldElement, DeclareTransactionResult)> {
    let _casm_class_hash = FieldElement::from_hex_be(
        "0x025dbb58db5071c88292cb25c81be128f2f47ccd8e3bd86260187f9937d181bb",
    )
    .unwrap();

    let casm_class = serde_json::from_reader::<_, CompiledClass>(std::fs::File::open(casm_path)?)?;

    let sierra_class =
        serde_json::from_reader::<_, SierraClass>(std::fs::File::open(sierra_path)?)?;

    let sierra_class_hash = sierra_class.class_hash().unwrap();
    let casm_class_hash = casm_class.class_hash().unwrap();

    let declaration = account.declare(Arc::new(sierra_class.flatten()?), casm_class_hash);
    let decl_res = declaration.send().await?;

    if let Some(interval) = watch_interval {
        utils::watch_tx(account.provider(), decl_res.transaction_hash, interval).await?;
    }

    Ok((sierra_class_hash, decl_res))
}
