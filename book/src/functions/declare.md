# Declare

Declare a `class-hash` on-chain.

```lua
declare("contract_name", opts)

-- @param contract_name - The contract name (string).
string

-- @param opts - Options for the transaction (table).
{
  -- The tx watch interval in milliseconds (or nil to not wait the tx receipt).
  watch_interval = number,
  -- The path to locate contract artifacts. For now, this path is relative
  -- to where you execute `kipt`. Be aware of that.
  artifacts_path = string,
  -- If the class is already declared, no error is returned, the declaration
  -- is skipped and the class hash is returned. The default value is false.
  skip_if_declared = bool,

  -- Any other keys in the table are ignored.
}

-- @return - A table on success, string error otherwise.
{
  -- The transaction hash (only if `skip_if_declared` is false).
  tx_hash = string,
  -- The declared class hash (Sierra class hash).
  class_hash = string,
}
```

There are two `class-hash` on Starknet:

- Sierra `class-hash`, which is the hash of the Sierra representation of the program, including the ABI.
- Casm `class-hash`, which is the compiled hash of the program.

By `class-hash` we regularly speak about the Sierra one. But to declare a contract, we also need the Casm `class-hash`
in order to ensure that the compilation of the Sierra code on-chain is lowered to the expected Casm `class-hash`.

To ease the process of declaring a new class, you must first use [Scarb](https://docs.swmansion.com/scarb/) to compile your contract.
Please ensure you've `sierra` and `casm` options enabled in your `Scarb.toml` file.

```toml
[[target.starknet-contract]]
# Enable Sierra codegen.
sierra = true
# Enable CASM codegen.
casm = true
```

This will generated two files (called contracts `artifacts`):

- `mycontract.contract_class.json` (Sierra)
- `mycontract.compiled_contract_class.json` (Casm)

The two files will be loaded by Kipt. For this, you only need to provide the contract's name and the path to find the artifacts
(usually generated in `target/dev` directory of your scarb package).

## Example

```lua
local opts = {
  watch_interval = 300,
  artifacts_path = "./target/dev",
}

local decl_res, _ = declare("mycontract", opts)

print("Declare transaction hash:" .. decl_res.tx_hash)
print("Declared class_hash: " .. decl_res.class_hash)

-- If you want to check the error and exit on error:

local decl_res, err = declare("mycontract", opts)

if err then
  print(err)
  -- Use a non-zero value to indicate an error.
  os.exit(1)
end
```

> ℹ️ **Note**
>
> Providing the path for artifacts and the contract's name, Kipt will search for `<contract_name>_contract_class.json` or `<contract_name>_sierra.json` for the Sierra file.
> And for the Casm file, Kipt will search for `<contract_name>_compiled_contract_class.json` or `<contract_name>_casm.json`.

As you can see, in few lines of code you can control the transactions sent on Starknet.
And having the `class-hash` from the declare, you can now easily [deploy](./deploy.md) an instance of the contract.
