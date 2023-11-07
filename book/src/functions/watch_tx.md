# Watch Transaction

Polls the receipt of the given transaction hash.

```lua
watch_tx("tx_hash", interval_ms)

-- @param tx_hash - The transaction hash (string).
string

-- @param interval_ms - The interval in milliseconds to poll the receipt (number).
number

```

As you've seen for transaction based functions (like [declare](./declare.md), [deploy](./deploy.md) and [invoke](./invoke.md))
you already have an option you can pass to wait for the transaction receipt before continuing.

However, in some cases, you may want to send several transactions, but you don't want [declare](./declare.md), [deploy](./deploy.md) and [invoke](./invoke.md) to block until the receipt is available.

For this, you can set the `watch_interval` of such functions to `nil` (or simply remove this key from the options), and then you can manually watch any transaction you would like.

## Example

```lua
-- Note here the absence of `watch_interval` key as options is empty.
local set_a_res, _ = invoke(
   {
      {
         to = "0x1111",
         func = "set_a",
         calldata = { "0x1234" },
      },
   },
   {}
)

-- the first invoke will not block, and will return
-- once the transaction hash is obtained from the RPC.
-- But the transaction may not be executed at the moment
-- we call this one.
local set_b_res, _ = invoke(
   {
      {
         to = "0x1111",
         func = "set_b",
         calldata = { "0xff" },
      },
   },
   {}
)

watch_tx(set_b_res.tx_hash, 200);

-- once here, we're sure that the second invoke transaction
-- was processed by the sequencer.
```
