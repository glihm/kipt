//! A simple logger used from lua,
//! but initialization is opaque for the user.
use chrono::Utc;
use mlua::{Lua, Result as LuaResult, Value};

pub fn setup(lua: &Lua) -> LuaResult<()> {
    lua.globals().set(
        "logger_init",
        lua.create_function(|lua, file_name: Option<String>| {
            let logger: Value = lua.globals().get("__INTERNAL_LOGGER__")?;
            if logger != Value::Nil {
                // Already initialized, do nothing.
                return Ok(logger);
            }

            let date = Utc::now().format("%A, %B %e, %Y %H:%M:%S").to_string();
            let logfile = file_name.unwrap_or("kipt.out".to_string());
            lua.load(format!(
                "__INTERNAL_LOGGER__ = io.open(\"{}\", \"w\")
__INTERNAL_LOGGER__:write(\"-- {} --\", \"\\n\\n\")
",
                logfile, date,
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
