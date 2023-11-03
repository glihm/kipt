# Kipt

Kipt is leveraging the simplicity of Lua scripts to manage Starknet contracts using `starknet-rs` under the hood.
With few lines, you can declare, deploy, invoke and call contracts.

The main focus of Kipt is to be used in the CI, without the need
to write bash script using Cast from Starknet Foundry or Starkli.

Under the hood, Kipt is using `starknet-rs` to interact with Starknet.

# Lua, an other language to learn?

You don't know Lua? No problem at all, it's a very small and easy scripting language, [beginner guide here](https://github.com/pohka/Lua-Beginners-Guide) and [full documentation here](https://www.lua.org/manual/5.4/manual.html). And to use Kipt, you only need to know very few element of the language.

If you prefer a short cheatsheet, go [here](https://devhints.io/lua) or [here](https://gist.github.com/nilesh-tawari/02078ae5b83ce3c90f476c4858c60693).

(For those who have already written an add-on for a famous MMO, welcome home!)

# Quick start

To test Kipt, the easiest way is:

1. Install Kipt
```bash
# 1. Install
curl https://raw.githubusercontent.com/glihm/kipt/main/kiptup/install | sh
source ~/.bashrc
kiptup
```
2. Create a simple "demo.lua" script copying the example below, replacing with the name of your contract.
3. Spin up katana in an other terminal.
4. Run `kipt --lua ./demo.lua`.

# Example

```lua
RPC = "KATANA"
ACCOUNT_ADDRESS = "0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973"
ACCOUNT_PRIVKEY = "0x1800000000300000180000000000030000000000003006001800006600"

-- No args -> kipt.out
local logger = logger_init()

local decl_res, err = declare(
    "mycontract",
    { watch_interval = 300, artifacts_path = "./target/dev" }
)

if err then
  print(err)
  os.exit(1)
end

print("Declared class_hash: " .. decl_res.class_hash)

-- Deploy with no constructor args.
local depl_res, err = deploy(decl_res.class_hash, {}, { watch_interval = 300, salt = "0x1234" })

if err then
  print(err)
  os.exit(2)
end

local contract_address = depl_res.deployed_address
print("Contract deployed at: " .. contract_address)

-- Invoke to set a value.
local invk_res, err = invoke(
   {
      {
         to = contract_address,
         func = "set_a",
         calldata = { "0x1234" },
      },
   },
   { watch_interval = 300 }
)

if err then
  print(err)
  os.exit(3)
end

print("Invoke TX hash: " .. invk_res.tx_hash)

local call_res, err = call(contract_address, "get_a", {}, {})
print_str_array(call_res)
```
