# Kipt

Kipt is leveraging the simplicity of Lua scripts to manage Starknet contracts using `starknet-rs` under the hood.
With few lines, you can declare, deploy, invoke and call contracts.

The main focus of Kipt is to be used in the CI, without the need
to write bash script using Cast from Starknet Foundry or Starkli.

Under the hood, Kipt is using `starknet-rs` to interact with Starknet.

# Lua, an other language to learn?

You don't know Lua? No problem at all, it's a very small and easy scripting language, [guide here](https://www.lua.org/manual/5.4/manual.html). And to use Kipt, you only need to know very few element of the language.

If you prefer a short cheatsheet, go [here](https://devhints.io/lua) or [here](https://gist.github.com/nilesh-tawari/02078ae5b83ce3c90f476c4858c60693).

(For those who have already written an add-on for World of Warcraft, welcome home!)

# Example

```lua
RPC="https://..."
ACCOUNT="0x..."
PRIVKEY="0x..."

local declare_opts = {
    watch_interval = 500,
}

local res, err = declare("/path/contract.sierra.json", "/path/contract.casm.json", declare_opts)

if err then
    print(err)
    os.exit()
end

print(res.tx_hash)
print(res.class_hash)
```

# TODO

- [ ] Add automatic logging into file to output this in CI as markdown.
- [ ] Add invoke
- [ ] Add call
- [ ] Add deploy
- [ ] Add dry-run with only estimation
