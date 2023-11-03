# Deploy

Deploy a contract instance for the given `class-hash`.

```lua
deploy("class_hash", args, opts)

-- @param class_hash - The sierra class hash to deploy (string).
string

-- @param args - Arguments passed to the constructor during deployment (table array-like of strings).
{ string, string, ... }

-- @param opts - Options for the transaction (table).
{
  -- The tx watch interval in milliseconds (or nil to not wait the tx receipt).
  watch_interval = number,
  -- The salt use to compute the contract address (or nil to use a random salt).
  salt = string,
  -- Any other keys in the table are ignored.
}

-- @return - A table on success, string error otherwise.
{
  -- The transaction hash.
  tx_hash = string,
  -- The address of the deployed contract.
  deployed_address = string,
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

local decl_res, _ = declare("mycontract", opts)
local class_hash = decl_res.class_hash

-- If the constructor has no arguments, just use an emtpy table.
local args = {}
local depl_res, _ = deploy(class_hash, args, opts)

print("Deploy transaction hash:" .. depl_res.tx_hash)
print("Deployed address: " .. depl_res.deployed_address)

-- Add some arguments in array-like fashion:
local args = { "0x1234", "0x8822" }
local depl_res, _ = deploy(class_hash, args, opts)
```
