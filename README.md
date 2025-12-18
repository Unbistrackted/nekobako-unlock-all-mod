# nekobako-unlock-all-mod ![Version](https://img.shields.io/github/v/release/Unbistrackted/nekobako-unlock-all-mod?style=plastic&label=Version&color=dc3e3e) ![Downloads](https://img.shields.io/github/downloads/Unbistrackted/nekobako-unlock-all-mod/total?style=plastic&label=Downloads&color=50f63f) 

A plugin for [Skyline](https://github.com/skyline-dev/skyline) that changes the "UNLOCK ALL" behaviour to actually unlock everything in ``Umineko no Naku Koro ni Saku ~Nekobako to Musou no Koukyoukyoku~``

``Title ID:01006A300BA2C000``


## Configuration Settings

 Setting Key                   | Value Type | Default Value | Description                                                                                    
-------------------------------|------------|---------------|------------------------------------------------------------------------------------------------
 is_enabled                    | bool       | true          | Enables or disables the plugin     


## Download

❗️This plugin only works with **UPDATE VERSION** ``0.0.3.0/v196608`` (You can check the version by pressing ``+`` on the game, at the top-left)

You can download the latest version of nekobako-unlock-all-mod [here](https://github.com/Unbistrackted/nekobako-unlock-all-mod/releases/latest).

After downloading, drop the contents of the .zip file into the root of your SD card.

You can find the config file at ``/atmosphere/contents/01006A300BA2C000/romfs/skyline/config/nekobako-unlock-all-mod/config.yaml``


## Building from source

To build from source you would need to have [Rust](https://www.rust-lang.org/) and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) installed.

Then you can build the plugin with:

```bash
cargo skyline build
```

Or generate the plugin package with:

```bash
cargo skyline package
```

## Special thanks to:

https://github.com/DCNick3 for the tools he created, the time explaining how the basics works, and all sorts of things that would not have made this possible without his help.

( He is also re-implementating the engine if you want to check it out, [DCNick3/shin](https://github.com/DCNick3/shin) )
