# Call

Realizes a function call on Starknet.

```lua
call("contract_address", "function_name", args, opts)

-- @param contract_address - The contract to target (string).
string

-- @param function_name - The name of the function to call (string).
string

-- @param args - Arguments passed to function (table array-like of strings).
{ string, string, ... }

-- @param opts - Options for the transaction (table).
{
  -- The block id against which the function call is done. Can be "pending", "latest" or any number in decimal.
  -- Default = "pending".
  block_id = string,
  -- Any other keys in the table are ignored.
}

-- @return - A table array-like of strings on success, string error otherwise.
{ string, string, ... }
```

For now, the `felt252` type is represented as a string in Lua. So you have to pass the arguments
as serialized felts.

Work in progress: in the future, Kipt will also provider some basic [scheme as starkli does](https://book.starkli.rs/argument-resolution#schemes).

The output of the call is also the serialized list of felts as `string`.

## Example

```lua
local call_res, _ = call(contract_address, "get_a", {}, { block_id = "latest" })
print_str_array(call_res)
```

> ℹ️ **Note**
>
> Note the usage of `print_str_array`, this is a function provided by Kipt to easily print a formatted array of felts returns by a call.
