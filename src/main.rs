//! Main file runing kipt.
//!
mod account;
mod args;
mod declare;
mod error;
mod lua;
mod utils;

fn main() {
    // Get clap args.

    lua::execute(
        r#"
RPC="http://0.0.0.0:5050"
ACCOUNT_ADDRESS="0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973"
ACCOUNT_PRIVKEY="0x1800000000300000180000000000030000000000003006001800006600"

local dopts = { watch_interval = 300 }
local d, err = declare("./contracts/artifacts/c1.sierra.json", "./contracts/artifacts/c1.casm.json", dopts)

if err then
  print(err)
  os.exit()
end

print(d.tx_hash)
print(d.class_hash)

-- No opts.
local _, _ = declare("./contracts/artifacts/c1.sierra.json", "./contracts/artifacts/c1.casm.json", {})

"#,
    )
    .unwrap();
}
