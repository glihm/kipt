# Installation and run

The easiest way to install Kipt is to use `kiptup`, a portable script that downloads prebuilt binaries and manages shell configuration for you. However, it might not be available depending on your device's platform and/or architecture.

## Using `kiptup`

If you're on Linux/macOS/WSL, you can install `kiptup` by running the following command:

```console
curl https://raw.githubusercontent.com/glihm/kipt/main/kiptup/install | sh
```

You might need to restart your shell session for the `kiptup` command to become available. Once it's available, run the `kiptup` command:

```console
kiptup
```

Running the commands installs `kipt` for you, and upgrades it to the latest release if it's already installed.

> ℹ️ **Note**
>
> Over time, `kiptup` itself may change and require upgrading. To upgrade `kiptup` itself, run the `curl` command above again.

## Prebuilt binaries

Prebuilt binaries are available with [GitHub releases](https://github.com/glihm/kipt/releases) for certain platforms.

Prebuilt binaries are best managed with [`kiptup`](#using-kiptup). However, if you're on a platform where `kiptup` isn't available (e.g. using `kiptup` on Windows natively), you can manually download the prebuilt binaries and make them available from `PATH`.

## Run Kipt

For now Kipt as very few options, and you only have to provide the Lua script to execute:

```console
kipt --lua ./scripts/demo.lua
```
