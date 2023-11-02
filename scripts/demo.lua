RPC="http://0.0.0.0:5050"
ACCOUNT_ADDRESS="0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973"
ACCOUNT_PRIVKEY="0x1800000000300000180000000000030000000000003006001800006600"

local opts = {
   watch_interval = 300,
   artifacts_path = "./contracts/artifacts",
}

local decl_res, err = declare("c1", opts)

if err then
  print(err)
  os.exit()
end

-- print(decl_res.tx_hash)
print("Declared class_hash: " .. decl_res.class_hash)

-- Deploy with no constructor args and no options.
local args = {}
local depl_res, err = deploy(decl_res.class_hash, args, opts)

if err then
  print(err)
  os.exit()
end

-- print(depl_res.tx_hash)
print("Contract deployed at: " .. depl_res.deployed_address)
