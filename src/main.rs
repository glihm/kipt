//! Main file runing kipt.
//!
mod account;
mod args;
mod declare;
mod deploy;
mod error;
mod lua;
mod utils;

fn main() {
    // Get clap args.

    // Pass the lua file content instead of plain content.

    lua::execute(
        r#"
RPC="http://0.0.0.0:5050"
ACCOUNT_ADDRESS="0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973"
ACCOUNT_PRIVKEY="0x1800000000300000180000000000030000000000003006001800006600"

local dopts = { watch_interval = 300 }
local decl_res, err = declare("./contracts/artifacts/c1.sierra.json", "./contracts/artifacts/c1.casm.json", dopts)

if err then
  print(err)
  os.exit()
end

-- print(decl_res.tx_hash)
print("Declared class_hash: " .. decl_res.class_hash)

-- Deploy with no constructor args and no options.
local depl_res, err = deploy(decl_res.class_hash, {}, {})

if err then
  print(err)
  os.exit()
end

-- print(depl_res.tx_hash)
print("Contract deployed at: " .. depl_res.deployed_address)

"#,
    )
    .unwrap();
}
