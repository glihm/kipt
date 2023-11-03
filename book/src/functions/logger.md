# Logger

To ensure that all the transactions are recorded automatically,
you have to initialize the logger and Kipt will generated a report for you.

```lua
-- Without arguments, this will output everything in the file `kipt.out`.
local logger = logger_init()

-- With the name you may prefer:
local logger = logger_init("my_output.txt")
```

> ⚠️ **Warning**
>
> Kipt will overwrite the file if it already exist. Ensure to provide a different name to avoid data loss.

> ℹ️ **Note**
>
> Calling `logger_init()` multiple times with or without an argument will take effect only at the first call. All other calls are ignored.

By default, Kipt will write in the output file any transaction hash and output of the `declare`, `deploy` and `invoke` functions.
If you want to output additional information, you can do the following:

```lua
logger:write("your content here")
```
