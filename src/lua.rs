use lazy_static::lazy_static;
use mlua::{Error as LuaError, Lua, Result as LuaResult, Table};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};

use crate::error::ErrorExtLua;
use crate::{declare, deploy};

/// A simple trait to ensure that all
/// data returned from a lua function can be serialized
/// into a lua table.
pub trait LuaTableSetable {
    fn set_all(&self, table: &Table);
}

/// A structure that is returned from every lua function
/// wrapping a rust function for starknet.
pub struct LuaOutput<T: LuaTableSetable + Send> {
    pub is_success: bool,
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

    setup_starknet_funcs(&lua)?;
    lua.load(program).exec()?;

    Ok(())
}

/// Setups all starknet functions into the lua globals.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
fn setup_starknet_funcs(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    globals.set("NETWORK", "GOERLI-1")?;

    lua.globals().set(
        "declare",
        lua.create_function(
            |lua, (sierra_path, casm_path, options): (String, String, Table)| {
                Ok(declare::lua_declare(lua, sierra_path, casm_path, options))
            },
        )?,
    )?;

    lua.globals().set(
        "deploy",
        lua.create_function(
            |lua, (sierra_class_hash, args, options): (String, Vec<String>, Table)| {
                Ok(deploy::lua_deploy(lua, sierra_class_hash, args, options))
            },
        )?,
    )?;

    Ok(())
}

/// Retrieves account/provider/network information from Lua globals.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
pub fn get_account(lua: &Lua) -> LuaResult<(String, String, String)> {
    let url_network: Option<String> = lua.globals().get("RPC")?;
    let address: Option<String> = lua.globals().get("ACCOUNT_ADDRESS")?;
    let privkey: Option<String> = lua.globals().get("ACCOUNT_PRIVKEY")?;

    match (url_network, address, privkey) {
        (Some(un), Some(a), Some(p)) => Ok((un, a, p)),
        _ => Err(LuaError::ExternalError(std::sync::Arc::new(
            ErrorExtLua::new("RPC / ACCOUNT_ADDRESS / ACCOUNT_PRIVKEY must be set"),
        ))),
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
