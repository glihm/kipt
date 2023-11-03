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
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::error::{Error, ErrorExtLua, KiptResult};
use crate::lua::{self, LuaOutput, LuaTableSetable, RT};
use crate::{account, logger, utils};

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
    contract_name: String,
    options: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    let (url_network, address, privkey) = lua::get_account(lua)?;
    let artifacts_path: Option<String> = options.get("artifacts_path")?;
    let is_recursive: bool = options.get("artifacts_recursively")?;
    let watch_interval = lua::get_watch_from_options(&options)?;

    let mut out_log = String::from(&format!("> declare: {}\\n", contract_name));

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

            let (sierra_path, casm_path) = match locate_artifacts(
                &contract_name,
                &artifacts_path.unwrap_or("./".to_string()),
                is_recursive,
            ) {
                Ok((s, c)) => (s, c),
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

            out_log.push_str(&format!(
                "|     tx_hash      |  {}  |\\n",
                d.transaction_hash
            ));
            out_log.push_str(&format!(
                "|    class_hash    |  {}  |\\n",
                d.sierra_class_hash
            ));
            logger::write(lua, &out_log)?;
        }

        Ok(t)
    } else {
        out_log.push_str(&format!("error: {}\\n", data.error));

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

    // TODO: if the file is not found, the error returned by file::open is not giving the name.
    // we might consider adding this somehow to have a more explicit error.
    let casm_class = serde_json::from_reader::<_, CompiledClass>(std::fs::File::open(casm_path)?)?;

    let sierra_class =
        serde_json::from_reader::<_, SierraClass>(std::fs::File::open(sierra_path)?)?;

    // TODO: check first if class already declared. If yes, return the sierra_class_hash
    // and no other data, to let the user continue if already declared.

    let sierra_class_hash = sierra_class.class_hash().unwrap();
    let casm_class_hash = casm_class.class_hash().unwrap();

    let declaration = account.declare(Arc::new(sierra_class.flatten()?), casm_class_hash);
    let decl_res = declaration.send().await?;

    if let Some(interval) = watch_interval {
        utils::watch_tx(account.provider(), decl_res.transaction_hash, interval).await?;
    }

    Ok((sierra_class_hash, decl_res))
}

/// Locates the artifacts of a contract from it's name.
///
/// # Arguments
///
/// * `contract_name` - Name of the contract that will be tested as being the first part of the filename.
/// * `artifacts_dir` - The directory where to search for the files.
/// * `is_recursive` - If the search must be done recursively.
fn locate_artifacts(
    contract_name: &str,
    artifacts_dir: &str,
    is_recursive: bool,
) -> KiptResult<(String, String)> {
    let sierra_exts = ["contract_class.json", "sierra.json"];
    let casm_exts = ["compiled_contract_class.json", "casm.json"];

    let mut sierra_path: Option<String> = None;
    let mut casm_path: Option<String> = None;

    let dir = PathBuf::from(artifacts_dir);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() && is_recursive {
            return locate_artifacts(contract_name, &entry_path.to_string_lossy(), is_recursive);
        } else if let Some(file_name) = entry_path.file_name() {
            let fname = file_name.to_string_lossy();
            if !fname.starts_with(contract_name) {
                continue;
            }

            if sierra_exts.iter().any(|ext| fname.ends_with(ext)) {
                sierra_path = Some(entry_path.canonicalize()?.to_string_lossy().to_string());
            } else if casm_exts.iter().any(|ext| fname.ends_with(ext)) {
                casm_path = Some(entry_path.canonicalize()?.to_string_lossy().to_string());
            }
        }
    }

    match (sierra_path, casm_path) {
        (Some(s), Some(c)) => Ok((s.to_string(), c.to_string())),
        (None, _) => {
            // TODO: add a detail of file name that were looked for + a detail of directories?
            // Perhaps only with `trace!` level.
            Err(Error::ArtifactsMissing(format!(
                "Sierra artifacts not found for contract {}",
                contract_name
            )))
        }
        (_, None) => Err(Error::ArtifactsMissing(format!(
            "Casm artifacts not found for contract {}",
            contract_name
        ))),
    }
}
