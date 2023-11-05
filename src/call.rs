use mlua::{Error as LuaError, Lua, Result as LuaResult, Table};
use regex::Regex;
use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, FunctionCall},
    core::utils::get_selector_from_name,
    providers::{AnyProvider, Provider},
};

use crate::account;
use crate::error::{Error, ErrorExtLua, KiptResult};
use crate::lua::{self, LuaOutput, LuaTableSetable, RT};

/// Call output.
struct CallOutput {
    pub data: Vec<String>,
}

impl LuaTableSetable for CallOutput {
    fn set_all(&self, table: &Table) {
        table.set("data", self.data.clone()).unwrap();
    }
}

/// Defines a lua function that make a function call to a contract.
///
/// # Arguments
///
/// * `lua` - Lua VM instance.
/// * `contract_address` - The contract deployed contract address to call.
/// * `function_name` - The function name to convert into a selector.
/// * `calldata` - The function call arguments.
/// * `options` - Options for the call.
pub fn lua_call<'lua>(
    lua: &'lua Lua,
    contract_address: String,
    function_name: String,
    calldata: Vec<String>,
    options: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    let url_network = lua::get_provider(lua)?;

    let block_id: Option<String> = options.get("block_id")?;

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

            match function_call(
                &provider,
                &contract_address,
                &function_name,
                calldata,
                &block_id.unwrap_or("pending".to_string()),
            )
            .await
            {
                Ok(call_res) => LuaOutput {
                    data: Some(CallOutput { data: call_res }),
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

        // Lua idx starts to 1, sadly.
        let mut idx = 1;
        for v in d.data {
            t.set(idx, v)?;
            idx += 1;
        }

        Ok(t)
    } else {
        Err(LuaError::ExternalError(std::sync::Arc::new(
            ErrorExtLua::new(&data.error),
        )))
    }
}

/// Sends an invoke transaction to a contract.
///
/// # Arguments
///
/// * `provider` - The provider to make the function call.
/// * `contract_address` - The deployed contract address.
/// * `function_name` - Name of the function to be executed.
/// * `calldata` - The call data felts to pass as argument to the function.
/// * `block_id` - The block id against which the function call is made.
async fn function_call(
    provider: &AnyProvider,
    contract_address: &str,
    function_name: &str,
    calldata: Vec<String>,
    block_id: &str,
) -> KiptResult<Vec<String>> {
    let mut sn_calldata = vec![];

    for c in calldata {
        sn_calldata.push(FieldElement::from_hex_be(&c)?);
    }

    let r = provider
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(contract_address)?,
                entry_point_selector: get_selector_from_name(function_name)?,
                calldata: sn_calldata,
            },
            parse_block_id(block_id)?,
        )
        .await?;

    Ok(r.into_iter().map(|f| format!("0x{:064x}", f)).collect())
}

/// Function from starkli: https://github.com/xJonathanLEI/starkli/blob/3fd85cf58f7adf757f3a62a86ae4e1fe487b4c8a/src/utils.rs#L59C1-L71C2
pub fn parse_block_id(id: &str) -> KiptResult<BlockId> {
    let regex_block_number = Regex::new("^[0-9]{1,}$").unwrap();

    if id == "latest" {
        Ok(BlockId::Tag(BlockTag::Latest))
    } else if id == "pending" {
        Ok(BlockId::Tag(BlockTag::Pending))
    } else if regex_block_number.is_match(id) {
        Ok(BlockId::Number(id.parse::<u64>().map_err(|_e| {
            Error::Other(format!("Can't convert block_id as u64: {}", id))
        })?))
    } else {
        Ok(BlockId::Hash(FieldElement::from_hex_be(id)?))
    }
}
