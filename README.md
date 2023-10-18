# Blinkt

[![Build Status](https://travis-ci.com/golemparts/blinkt.svg?branch=master)](https://travis-ci.com/golemparts/blinkt)
[![crates.io](https://img.shields.io/crates/v/blinkt)](https://crates.io/crates/blinkt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.56.0-lightgray.svg)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

Blinkt is a Rust library that provides an interface for the Pimoroni Blinkt!, and any similar APA102 or SK9822 LED strips or boards, on a Raspberry Pi. The library supports bitbanging mode on any GPIO pins, and hardware SPI mode on GPIO 10 (physical pin 19) for data, and GPIO 11 (physical pin 23) for clock.

For bitbanging mode, Blinkt gains access to the BCM283x GPIO peripheral either through `/dev/gpiomem` or `/dev/mem`. Hardware SPI mode is controlled through `/dev/spidev0.0`.

Both the original APA102 and the SK9822 clone are supported. The RGB LED/driver ICs are referred to as pixels throughout the code and documentation.

Backwards compatibility for minor revisions isn't guaranteed until the library reaches v1.0.0.

Blinkt is under development on the [master branch](https://github.com/golemparts/blinkt/tree/master) of the repository on GitHub. If you're looking for the `README.md` or the `examples` directory for the latest release or any of the earlier releases, visit [crates.io](https://crates.io/crates/blinkt), download an archived release from the GitHub [releases](https://github.com/golemparts/blinkt/releases) page, or clone and checkout the relevant release tag.

## Documentation

Online documentation is available for the latest release, older releases, and the version currently in development.

* Latest release: [docs.golemparts.com/blinkt](https://docs.golemparts.com/blinkt)
* Older releases: [docs.rs/blinkt](https://docs.rs/blinkt)
* In development: [docs.golemparts.com/blinkt-dev](https://docs.golemparts.com/blinkt-dev)

## Usage

Add a dependency for `blinkt` to your `Cargo.toml` using `cargo add blinkt`, or by adding the following line to your dependencies section.

```toml
[dependencies]
blinkt = "0.7.1"
```

Call `Blinkt::new()` to create a new Blinkt with the default settings. Alternative configuration options are available through `Blinkt::with_settings()` and `Blinkt::with_spi()`.

```rust
use blinkt::Blinkt;

let mut blinkt = Blinkt::new()?;
```

## Examples

The example below demonstrates swapping all pixels on a Blinkt! board between red, green and blue.

```rust
use std::error::Error;
use std::time::Duration;
use std::{mem, thread};

use blinkt::Blinkt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    let (red, green, blue) = (&mut 255, &mut 0, &mut 0);

    loop {
        blinkt.set_all_pixels(*red, *green, *blue);
        blinkt.show()?;

        thread::sleep(Duration::from_millis(250));

        mem::swap(red, green);
        mem::swap(red, blue);
    }
}
```

To control an LED strip consisting of 144 pixels, connected to the Raspberry Pi's hardware SPI pins (data on GPIO 10 (physical pin 19), and clock on GPIO 11 (physical pin 23)), at 16 MHz clock speed, replace the `Blinkt::new()` line in the above example with the following. You may have to tweak the maximum clock speed based on the number of pixels and the wire quality.

```rust
let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;
```

Additional examples can be found in the `examples` directory.

## Cross compilation

If you're not working directly on a Raspberry Pi, you'll have to cross-compile your code for the appropriate ARM architecture. Check out [this guide](https://github.com/japaric/rust-cross) for more information, or try the [cross](https://github.com/japaric/cross) project for "zero setup" cross compilation.

### Cargo

While additional steps may be necessary to cross-compile binaries on your platform, checking your code with `cargo check` only requires the installation of an appropriate target. Most Raspberry Pi models need the `armv7-unknown-linux-gnueabihf` target. For some models, like the Raspberry Pi Zero, a different target triple is required.

Install the relevant target using `rustup`.

```bash
rustup target install armv7-unknown-linux-gnueabihf
```

In the root directory of your project, create a `.cargo` subdirectory, and then save the following snippet to `.cargo/config`.

```toml
[build]
target = "armv7-unknown-linux-gnueabihf"
```

### Visual Studio Code

The rust-analyzer extension for Visual Studio Code needs to be made aware of the target platform by setting the `rust-analyzer.cargo.target` configuration option. In the root directory of your project, create a `.vscode` subdirectory, and then save the following snippet to `.vscode/settings.json`.

```json
{
    "rust-analyzer.cargo.target": "armv7-unknown-linux-gnueabihf"
}
```

## Copyright and license

Copyright (c) 2016-2022 Rene van der Meer. Released under the [MIT license](LICENSE).
