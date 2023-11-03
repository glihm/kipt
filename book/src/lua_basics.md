# Basics

Some Lua and Kipt basics to get started with the scripting:

## Lua basics

In Lua, only [8 types are used](https://www.lua.org/manual/5.4/manual.html#:~:text=There%20are%20eight%20basic%20types,absence%20of%20a%20useful%20value.).
In the context of Kipt, you only need 5 of them if you want to keep it simple:

* `nil`: nothing / no value.
* `number`
* `string`
* `table`
* `boolean`

Except for `table`, the other types are very common.

You can notice that there are no `array`, but `table` is flexible enough to be used as `hashmap` and `array`.

```lua
-- nil
local a = nil

-- Number
local b = 1234

-- String
local c = "kipt"

-- Boolean
local d = false

-- Table
local e = { a = 2, b = "abc" }

-- Table (array-like)
local f = { 1, 2, 3, 4 }
```

The use of `local` is optional, but it's a good practice as any variable without `local` is considered as global.
So if you use some functions for more advanced Lua programming, keep the use of `local`.

In case of array, under the hood it's a `table`, but index starting to `1` (nobody is perfect...).

To access member of an `table`, you have two notations as showed below. If the `key` does not exist,
it returns `nil`.

```lua
local t = { a = 2, b = "abc" }
print(t.a)
print(t["b"])

if t.h then
    -- h is not nil and has a value, do stuff with it.
end
```

To test `nil`, the best practice is to compare with `nil`, this is because `nil` and `false` are evaluated the same way:

```lua
local a = false
local b = nil

if not a and not b then
    print("will be printed")
end

if a == nil and b == nil then
    print("will not be printed")
end
```

This are all the basics you need to get started.
