# Blinkt

[![crates.io](https://img.shields.io/crates/v/blinkt)](https://crates.io/crates/blinkt)
[![Documentation](https://docs.rs/blinkt/badge.svg)](https://docs.rs/blinkt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.56.0-lightgray.svg)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

## This project is no longer maintained

As of July 1, 2025, I have decided to retire Blinkt. This means:
* No new features will be added.
* Bug fixes will no longer be provided.
* Support for new hardware is not planned.
* Pull requests and issues will no longer be reviewed or addressed.

I want to express my sincere gratitude to everyone who contributed to, used, and supported Blinkt over the years. Your contributions and feedback were invaluable.

### Why have I retired Blinkt?

Blinkt began as a passion project in 2016, nearly nine years ago, when I first started working with electronics and needed a project to work on my Rust skills. 

However, over the past several years, my personal interests and professional focus have shifted away from electronics. As a result, I haven't actively used Blinkt myself for quite some time. I no longer have a dedicated hardware test setup, nor do I plan on adding new Raspberry Pi models to my collection. This makes it impractical to thoroughly test changes or ensure compatibility with new hardware releases.

Maintaining a project requires significant dedication, and without active personal use or the necessary testing environment, it's become challenging to provide the level of attention this project deserves.

### What does this mean for you?

You are welcome to continue using Blinkt. However, please be aware you will not receive any further updates or support.

#### Forking the project

If you wish to continue its development, you may fork this project under the terms and conditions of the MIT License.

## Blinkt

Blinkt is a Rust library that provides an interface for the Pimoroni Blinkt!, and any similar APA102 or SK9822 LED strips or boards, on a Raspberry Pi. The library supports bitbanging mode on any GPIO pins, and hardware SPI mode on GPIO 10 (physical pin 19) for data, and GPIO 11 (physical pin 23) for clock.

For bitbanging mode, Blinkt gains access to the BCM283x GPIO peripheral either through `/dev/gpiomem` or `/dev/mem`. Hardware SPI mode is controlled through `/dev/spidev0.0`.

Both the original APA102 and the SK9822 clone are supported. The RGB LED/driver ICs are referred to as pixels throughout the code and documentation.

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

Copyright (c) 2016-2025 Rene van der Meer. Released under the [MIT license](LICENSE).
