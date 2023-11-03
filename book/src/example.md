```lua
-- Initializing some variables. RPC, ACCOUNT_ADDRESS and ACCOUNT_PRIVKEY are known by Kipt.
RPC="http://0.0.0.0:5050"
ACCOUNT_ADDRESS="0x1234..."
ACCOUNT_PRIVKEY="0x111...."

-- Configuring some options:
local opts = {
    -- watch any transaction every 300 ms
    watch_interval = 300,
    -- path to looks for artifacts of the contracts
    artifacts_path = "./target/dev",
}

-- Declaring the contract "demo_contract1"
local decl, err = declare("demo_contract1", opts)

-- If you don't want to continue on error, add a little test.
if err then
    print(err)
    os.exit()
end

-- Deploying the class just declared, without any arguments passed to the constructor:
local args = {}
local depl, err = deploy(decl.class_hash, args, opts)

if err then
    print(err)
    os.exit()
end

-- Sending some transactions, multicall is supported.
local invk, err = invoke(
   {
      {
         to = depl.deployed_address,
         func = "set_a",
         calldata = { "0x1234" },
      },
   },
   opts
)
```
