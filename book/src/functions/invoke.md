# Invoke

Sends an invoke transaction to Starknet.

```lua
invoke(calls, opts)

-- @param calls - A list of calls to be executed (table).
{
  {
    -- Contract to target.
    to = string,
    -- The function name to invoke.
    func = string,
    -- Arguments for the function (table array-like of strings)
    calldata = { string, string ... },
  },
  ...
}

-- @param opts - Options for the transaction (table).
{
  -- The tx watch interval in milliseconds (or nil to not wait the tx receipt).
  watch_interval = number,
  -- Any other keys in the table are ignored.
}

-- @return - A table on success, string error otherwise.
{
  -- The transaction hash.
  tx_hash = string,
}
```

For now, the `felt252` type is represented as a string in Lua. So you have to pass the arguments
as serialized felts.

Work in progress: in the future, Kipt will also provider some basic [scheme as starkli does](https://book.starkli.rs/argument-resolution#schemes).

## Example

```lua
local opts = {
  watch_interval = 300,
}

local invk_res, _ = invoke(
   {
      {
         to = "0x1111",
         func = "set_a",
         calldata = { "0x1234" },
      },
   },
   opts
)

print("Invoke TX hash: " .. invk_res.tx_hash)
```
