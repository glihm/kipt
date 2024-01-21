use mlua::{Error as LuaError, Lua, Result as LuaResult, Table};
use starknet::{
    accounts::{ConnectedAccount, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{FieldElement, InvokeTransactionResult},
    providers::AnyProvider,
    signers::{LocalWallet, SigningKey},
};
use std::sync::Arc;
use std::time::Duration;

use crate::error::{ErrorExtLua, KiptResult};
use crate::lua::{self, LuaOutput, LuaTableSetable, RT};
use crate::{account, logger, transaction};

/// Deploy output.
struct DeployOutput {
    pub transaction_hash: String,
    pub deployed_address: String,
}

impl LuaTableSetable for DeployOutput {
    fn set_all(&self, table: &Table) {
        table.set("tx_hash", self.transaction_hash.clone()).unwrap();

        table
            .set("deployed_address", self.deployed_address.clone())
            .unwrap();
    }
}

/// Defines a lua function that deploys a contract.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
/// * `sierra_class_hash` - Contract class hash.
/// * `args` - Constructor arguments.
/// * `options` - Options for the deploy transaction.
pub fn lua_deploy<'lua>(
    lua: &'lua Lua,
    sierra_class_hash: String,
    args: Vec<String>,
    options: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    let (url_network, address, privkey, is_legacy) = lua::get_account(lua)?;

    let watch_interval = lua::get_watch_from_options(&options)?;
    let salt: Option<String> = options.get("salt")?;

    let mut out_log = String::from(&format!("> deploy: {}\\n", sierra_class_hash));

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

            match deploy_tx(account, &sierra_class_hash, &args, salt, watch_interval).await {
                Ok((deployed_address, depl_res)) => LuaOutput {
                    data: Some(DeployOutput {
                        transaction_hash: format!("0x{:064x}", depl_res.transaction_hash),
                        deployed_address: format!("0x{:064x}", deployed_address),
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
        out_log.push_str(&format!(
            "| deployed address |  {}  |\\n",
            d.deployed_address
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

/// Sends a transaction to deploy a contract.
///
/// # Arguments
///
/// * `account` - The account used to sign and send the transaction.
/// * `sierra_class_hash` - Contract class hash.
/// * `args` - Constructor arguments.
/// * `salt` - Optional salt for contract address computation. A random value
///   is used if `None` is provided.
/// * `watch_interval` - Watch interval for the transaction receipt.
async fn deploy_tx(
    account: SingleOwnerAccount<AnyProvider, LocalWallet>,
    sierra_class_hash: &str,
    args: &[String],
    salt: Option<String>,
    watch_interval: Option<Duration>,
) -> KiptResult<(FieldElement, InvokeTransactionResult)> {
    let class_hash = FieldElement::from_hex_be(sierra_class_hash)?;

    let mut ctor_args: Vec<FieldElement> = vec![];
    for a in args {
        ctor_args.push(FieldElement::from_hex_be(a)?);
    }

    let salt = if let Some(s) = salt {
        FieldElement::from_hex_be(&s)?
    } else {
        SigningKey::from_random().secret_scalar()
    };

    let account = Arc::new(account);
    let factory = ContractFactory::new(class_hash, Arc::clone(&account));

    let is_unique = false;
    let contract_deployment = factory.deploy(ctor_args, salt, is_unique);
    let deployed_address = contract_deployment.deployed_address();

    let depl_res = contract_deployment.send().await?;

    if let Some(interval) = watch_interval {
        transaction::poll_exec_succeeded(account.provider(), depl_res.transaction_hash, interval)
            .await?;
    }

    Ok((deployed_address, depl_res))
}
