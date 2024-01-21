use lazy_static::lazy_static;
use mlua::{Function, Lua, Number, Result as LuaResult, Table};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};

use crate::{call, declare, deploy, invoke, invoke::InvokeCall, logger, transaction};

/// A simple trait to ensure that all
/// data returned from a lua function can be serialized
/// into a lua table.
pub trait LuaTableSetable {
    fn set_all(&self, table: &Table);
}

impl LuaTableSetable for () {
    fn set_all(&self, _table: &Table) {}
}

/// A structure that is returned from every lua function
/// wrapping a rust function for starknet.
pub struct LuaOutput<T: LuaTableSetable + Send> {
    pub data: Option<T>,
    pub error: String,
}

lazy_static! {
    pub static ref RT: Runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Should create a tokio runtime");
}

/// Executes a lua program with a dedicated Lua context.
/// The starknet context is automatically injected
/// before intepreting the input program.
///
/// # Arguments
///
/// * `program` - Lua program to be executed.
pub fn execute(program: &str) -> LuaResult<()> {
    let lua = Lua::new();

    logger::setup(&lua)?;

    setup_starknet_funcs(&lua)?;

    lua.load(program).exec()?;

    logger::close(&lua)?;

    Ok(())
}

/// Setups all starknet functions into the lua globals.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
fn setup_starknet_funcs(lua: &Lua) -> LuaResult<()> {
    lua.globals().set(
        "get_logger",
        lua.create_function(|lua, ()| {
            let logger: mlua::Value = lua.globals().get("__INTERNAL_LOGGER__")?;
            Ok(logger)
        })?,
    )?;

    lua.globals().set(
        "print_str_array",
        lua.create_function(|lua, arr: Vec<String>| {
            let mut out = String::new();
            out.push('[');

            for (i, s) in arr.iter().enumerate() {
                out.push_str(s);
                if i < arr.len() - 1 {
                    out.push_str(", ");
                }
            }

            out.push(']');

            let print: Function = lua.globals().get("print")?;
            print.call::<_, _>(out)?;

            Ok(())
        })?,
    )?;

    lua.globals().set(
        "declare",
        lua.create_function(|lua, (contract_name, options): (String, Table)| {
            Ok(declare::lua_declare(lua, contract_name, options))
        })?,
    )?;

    lua.globals().set(
        "deploy",
        lua.create_function(
            |lua, (sierra_class_hash, args, options): (String, Vec<String>, Table)| {
                Ok(deploy::lua_deploy(lua, sierra_class_hash, args, options))
            },
        )?,
    )?;

    lua.globals().set(
        "invoke",
        lua.create_function(|lua, (calls, options): (Vec<InvokeCall>, Table)| {
            Ok(invoke::lua_invoke(lua, calls, options))
        })?,
    )?;

    lua.globals().set(
        "call",
        lua.create_function(
            |lua,
             (contract_address, function_name, calldata, options): (
                String,
                String,
                Vec<String>,
                Table,
            )| {
                Ok(call::lua_call(
                    lua,
                    contract_address,
                    function_name,
                    calldata,
                    options,
                ))
            },
        )?,
    )?;

    lua.globals().set(
        "watch_tx",
        lua.create_function(|lua, (transaction_hash, interval_ms): (String, Number)| {
            Ok(transaction::lua_watch(lua, transaction_hash, interval_ms))
        })?,
    )?;

    Ok(())
}

/// Retrieves account/provider/network information from Lua globals.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
pub fn get_account(lua: &Lua) -> LuaResult<(String, String, String, bool)> {
    let url_network: Option<String> = lua.globals().get("RPC")?;
    let address: Option<String> = lua.globals().get("ACCOUNT_ADDRESS")?;
    let privkey: Option<String> = lua.globals().get("ACCOUNT_PRIVKEY")?;
    let is_legacy: Option<bool> = lua.globals().get("ACCOUNT_IS_LEGACY")?;

    match (url_network, address, privkey) {
        (Some(un), Some(a), Some(p)) => {
            if let Some(true) = is_legacy {
                Ok((un, a, p, true))
            } else {
                Ok((un, a, p, false))
            }
        }
        _ => {
            // Without RPC and account info, we can't send tx. Panic here.
            panic!(
                r#"
RPC, ACCOUNT_ADDRESS and ACCOUNT_PRIVKEY variables were required by a transaction, but one of them (or all) is not provided.
Please consider setting RPC, ACCOUNT_ADDRESS and ACCOUNT_PRIVKEY variables at the top of you Lua script without the local keyword.

RPC = "https://...."
ACCOUNT_ADDRESS = "0x123..."
ACCOUNT_PRIVKEY = "0x987..."
"#,
            );
        }
    }
}

/// Retrieves only the network / rpc from Lua globals.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
pub fn get_provider(lua: &Lua) -> LuaResult<String> {
    let url_network: Option<String> = lua.globals().get("RPC")?;

    match url_network {
        Some(un) => Ok(un),
        _ => {
            // Without RPC, we can't make call. Panic here.
            panic!(
                r#"
RPC variable was required by a call, but it's not provided.
Please consider setting RPC variable at the top of you Lua script
without the local keyword.

RPC = "https://...."
"#,
            );
        }
    }
}

/// Retrieves the watch interval that may be present in the given lua table.
///
/// # Arguments
///
/// * `table` - Lua table that may contain "watch_interval" key.
pub fn get_watch_from_options(table: &Table) -> LuaResult<Option<Duration>> {
    let o: Option<u32> = table.get("watch_interval")?;
    if let Some(interval) = o {
        Ok(Some(Duration::from_millis(interval.into())))
    } else {
        Ok(None)
    }
}
