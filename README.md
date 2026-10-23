
# NetManagerDiag
![Banner](./.github/assets/NetManagerBanner.png)

The baseband diagnostic core for [NetManager](https://github.com/DottoXD/NetManager).


[![License: GPL v3](./.github/assets/license.svg)](https://gnu.org)
[![Supported platform: Android](./.github/assets/android.svg)](https://android.com)
[![Built with Rust](./.github/assets/rust.svg)](https://rust-lang.org/)

## Building & bundling
You can build NetManagerDiag with the following commands.
```
TODO
```

You can review an example of how to bundle NetManagerDiag in [NetManager's Android build.gradle.kts](https://github.com/DottoXD/NetManager/blob/main/android/app/build.gradle.kts).

## Why, what, how?
NetManagerDiag was created specifically to add baseband cell monitoring features to [NetManager](https://github.com/DottoXD/NetManager).

The library was built from scratch in Rust to guarantee a performant, memory-safe experience which could be easily bundled in an Android app.

NetManagerDiag currently supports all Qualcomm devices with either `/dev/diag` or `diag-router` access. MediaTek and Shannon devices are currently unsupported, and support for devices with other modems is currently unplanned.


The daemon opens the available interface, establishes a diagnostic session, configures the relevant modem log codes, and decodes supported messages into vendor-neutral events.

## Usage
TODO

## Tested SOCs
NetManagerDiag has been tested on all SOCs in the following table. If you have tested the library on one that isn't listed here, please report it in a [GitHub Issue](https://github.com/DottoXD/NetManagerDiag/issues) (and make sure to mention if it's working properly).

✔️ = Working properly, no issues.

⚠️ = Works with a few hiccups.

❌ = Not working at all or has substantial issues.


| SOC  | Status |
| ------------- | ------------- |
| Snapdragon 435 | ✔️ |
| Snapdragon 778G | ✔️ |

## AI statement
AI is being used for the following purposes during the development of NetManagerDiag:
- definiting how to structure the project and split things up
- cleaning up the code from bugs or ugly pieces of code caused by my little experience in Rust
- sometimes helped at building probes to map things out (to later implement into this library)

All code **must** be human-written or, **at most**, AI assisted (**NOT** AI written!) and then carefully reviewd and tested by a human. All contributions that are obviously AI written will be promptly rejected.

## Supported platforms & features
*Feature availability may vary by device, Android version, modem & carrier.*

| Feature / Platform  | Qualcomm (/dev/diag) | Qualcomm (diag-router) | MediaTek | Samsung Shannon | Unisoc |
| ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
| LTE serving cell measurements | ✔️ | ✔️ | ❌ | ❌ | ❌ |
| LTE cell information | ✔️ | ✔️ | ❌ | ❌ | ❌ |
| NR measurement table | ✔️ | ✔️ | ❌ | ❌ | ❌ |
| LTE bearers | ✔️ | ✔️ | ❌ | ❌ | ❌ |

## Issues and pull requests
Feel free to open an issue for any question, suggestion or any actual problem with NetManagerDiag.

Pull requests are highly appreciated as long as they're carefully tested.

## License
```
Copyright (C) 2026 DottoXD

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
