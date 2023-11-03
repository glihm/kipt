//! A simple logger used from lua,
//! but initialization is opaque for the user.
use mlua::{Lua, Result as LuaResult};

pub fn setup(lua: &Lua) -> LuaResult<()> {
    lua.globals().set(
        "logger_init",
        lua.create_function(|lua, file_name: Option<String>| {
            // TODO: add timestamp to avoid erased some data?
            let logfile = file_name.unwrap_or("kipt.out".to_string());
            lua.load(format!(
                "__INTERNAL_LOGGER__ = io.open(\"{}\", \"w\")",
                logfile
            ))
            .exec()?;

            let logger: mlua::Value = lua.globals().get("__INTERNAL_LOGGER__")?;
            Ok(logger)
        })?,
    )?;

    Ok(())
}

pub fn write(lua: &Lua, data: &str) -> LuaResult<()> {
    lua.load(format!("__INTERNAL_LOGGER__:write(\"{}\", \"\\n\")", data))
        .exec()?;

    Ok(())
}

pub fn close(lua: &Lua) -> LuaResult<()> {
    lua.load("__INTERNAL_LOGGER__:close()").exec()?;

    Ok(())
}
