# Kipt: easy contract management on Starknet

Kipt was develop to considerably ease the smart contracts management on Starknet.

"Ki" is from the Ki energy from a famous manga combined with the work "script", to illustrate the fact
that Kipt's simplicity empowers the user to easily script his contracts declare, deploy and transact operations.

Leveraging the simplicity of [Lua](https://www.lua.org/manual/5.3/manual.html) and the performance of Rust,
Kipt abstracts the whole complexity of interacting with the blockchain (with `starknet-rs`).

Basically, Kipt is capable of executing Lua code, where special functions are made available
out of the box for the user like `declare`, `deploy`, `invoke` and `call` functions related
to Starknet.
