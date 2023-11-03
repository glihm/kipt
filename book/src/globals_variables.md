# Global variables and setup

To interact with Rust, Kipt must retrieve some variables from Lua.
Any variable without the `local` keyword are accessible from Rust.

For this purposes, some variables are automatically checked by Kipt to add some configuration:

```lua
-- No local keyword, they are global variables.
RPC = "GOERLI-1"

ACCOUNT_ADDRESS = "0x1234...."
ACCOUNT_PRIVKEY = "0x8888...."

-- You can re-defined anywhere in the script a new value
RPC = "http://0.0.0.0:5050"
```

As shown, some global variables are expected by Kipt:

- `RPC`: can be any valid URL that starts with `http` or some pre-defined networks. The supported networks are:

  - `MAINNET`: to use the gateway for the mainnet.
  - `GOERLI-1`: to use the gateway for goerli-1.
  - `GOERLI-2`: to use the gateway for goerli-2.
  - `KATANA`: to use the Katana in local on the default port `http://0.0.0.0:5050`.

- `ACCOUNT_ADDRESS`: The address of the account to use to send transactions.
- `ACCOUNT_PRIVKEY`: The private key of the account to use to send transactions.

> ℹ️ **Note**
>
> Those variables are re-evaluated before **each** transaction. This means that you can change their
> value during the script without any problem to send some transactions with different accounts, to different URLs.

As Lua is a scripting language, it also has some capabilities. To not write your private key in plain text or pre-configured (CI for instance)
the account and network with environment variables, you can do the following:

```lua
RPC = os.getenv("STARKNET_RPC")
ACCOUNT_PRIVKEY = os.getenv("STARKNET_KEY")
```
