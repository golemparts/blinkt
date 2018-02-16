// Copyright (c) 2016-2018 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! A Rust library that provides an interface for the Pimoroni Blinkt!, and any
//! similar APA102 or SK9822 strips or boards, on a Raspberry Pi.
//!
//! Blinkt accesses the BCM283x GPIO peripheral either through `/dev/gpiomem`
//! (preferred) or `/dev/mem`. Both the original APA102 and the SK9822 clone
//! are supported. The APA102 RGB LED/driver ICs are referred to as pixels
//! throughout the code and documentation.
//!
//! Each pixel has a red, green and blue LED with possible values between 0-255.
//! Additionally, the overall brightness of each pixel can be set to 0.0-1.0, which
//! is converted to a 5-bit value.
//!
//! Blinkt stores all color and brightness changes in a local buffer. Use
//! `show()` to send the buffered values to the pixels.
//!
//! # Examples
//!
//! A complete example that cycles all pixels through red, green and blue.
//!
//! ```rust,no_run
//! extern crate blinkt;
//!
//! use std::{thread, mem};
//! use std::time::Duration;
//!
//! use blinkt::Blinkt;
//!
//! fn main() {
//!     let mut blinkt = Blinkt::new().unwrap();
//!     let (red, green, blue) = (&mut 255, &mut 0, &mut 0);
//!
//!     loop {
//!         blinkt.set_all_pixels(*red, *green, *blue);
//!         blinkt.show();
//!
//!         thread::sleep(Duration::from_millis(250));
//!
//!         mem::swap(red, green);
//!         mem::swap(red, blue);
//!     }
//! }
//! ```
//!
//! By default, all pixels are cleared when Blinkt goes out of
//! scope. Use `set_clear_on_drop(false)` to disable this behavior. Note that
//! drop methods aren't called when a program is abnormally terminated (for
//! instance when a SIGINT isn't caught).
//!
//! ```rust,no_run
//! use blinkt::Blinkt;
//!
//! let mut blinkt = Blinkt::new().unwrap();
//! blinkt.set_clear_on_drop(false);
//!
//! for n in 0..8 {
//!     blinkt.set_pixel(n, 36 * n as u8, 0, 255 - (36 * n as u8));
//! }
//!
//! blinkt.show();
//! ```
//!

#![recursion_limit = "128"] // Needed for the quick_error! macro

#[macro_use]
extern crate quick_error;
extern crate rppal;

use std::result;

use rppal::gpio::{Gpio, Level, Mode};

pub use rppal::gpio::Error as GpioError;

// Default values for the Pimoroni Blinkt! board using BCM GPIO pin numbers
const DAT: u8 = 23;
const CLK: u8 = 24;
const NUM_PIXELS: usize = 8;

const DEFAULT_BRIGHTNESS: u8 = 7;

quick_error! {
    #[derive(Debug)]
/// Errors that can occur when creating a new Blinkt.
    pub enum Error {
/// Accessing the GPIO peripheral returned an error.
///
/// Some of these errors can be fixed by changing file permissions, or upgrading
/// to a newer version of Raspbian.
        Gpio(err: GpioError) { description(err.description()) from() }
    }
}

/// Result type returned from methods that can have `blinkt::Error`s.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Copy, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    brightness: u8,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            red: 0,
            green: 0,
            blue: 0,
            brightness: DEFAULT_BRIGHTNESS,
        }
    }
}

/// Interface for a Blinkt! or any similar APA102 or SK9822 strips/boards.
///
/// By default, Blinkt is set up to communicate with an 8-pixel board through
/// data pin GPIO 23 and clock pin GPIO 24. These settings can be changed to
/// support alternate configurations.
pub struct Blinkt {
    gpio: Gpio,
    pixels: Vec<Pixel>,
    clear_on_drop: bool,
    pin_data: u8,
    pin_clock: u8,
    endframe_pulses: usize,
}

impl Blinkt {
    /// Creates a new `Blinkt` using the default settings for a Pimoroni
    /// Blinkt! board.
    ///
    /// This sets the data pin to GPIO 23, the clock pin to GPIO 24, and number
    /// of pixels to 8.
    pub fn new() -> Result<Blinkt> {
        Blinkt::with_settings(DAT, CLK, NUM_PIXELS)
    }

    /// Creates a new `Blinkt` using custom settings for the data pin, clock
    /// pin, and number of pixels. Pins should be specified by their BCM GPIO
    /// pin numbers.
    pub fn with_settings(pin_data: u8, pin_clock: u8, num_pixels: usize) -> Result<Blinkt> {
        // GPIO init might fail with an error the user could solve
        let mut gpio = try!(Gpio::new());

        gpio.set_mode(pin_data, Mode::Output);
        gpio.write(pin_data, Level::Low);
        gpio.set_mode(pin_clock, Mode::Output);
        gpio.write(pin_clock, Level::Low);

        Ok(Blinkt {
            gpio: gpio,
            pixels: vec![Pixel::default(); num_pixels],
            clear_on_drop: true,
            pin_data: pin_data,
            pin_clock: pin_clock,
            endframe_pulses: ((num_pixels as f32 * 0.5) + 0.5) as usize,
        })
    }

    /// When enabled, clears all pixels when the `Blinkt` goes out of scope.
    ///
    /// Drop methods aren't called when a program is abnormally terminated,
    /// for instance when a user presses Ctrl-C, and the SIGINT signal isn't
    /// caught. You'll either have to catch those using crates such as
    /// `simple_signal`, or manually call `cleanup()`.
    ///
    /// Enabled by default.
    pub fn set_clear_on_drop(&mut self, clear_on_drop: bool) {
        self.clear_on_drop = clear_on_drop;
    }

    /// Changes the GPIO pin mode for the data and clock pins back to their
    /// original state, and optionally clears all pixels.
    ///
    /// Normally, this method is automatically called when Blinkt goes out of
    /// scope, but you can manually call it to handle early/abnormal termination.
    /// After calling this method, any future calls to `show()` won't have any
    /// result.
    pub fn cleanup(&mut self) {
        if self.clear_on_drop {
            self.clear();
            self.show();
        }

        self.gpio.cleanup();
    }

    /// Sets the red, green and blue values for a single pixel in the local
    /// buffer.
    ///
    /// For an 8-pixel board, valid values for pixel are 0-7. Valid values
    /// for red, green and blue are 0-255.
    pub fn set_pixel(&mut self, pixel: usize, red: u8, green: u8, blue: u8) {
        if let Some(pixel) = self.pixels.get_mut(pixel) {
            pixel.red = red;
            pixel.green = green;
            pixel.blue = blue;
        }
    }

    /// Sets the red, green, blue and brightness values for a single pixel in
    /// the local buffer.
    ///
    /// For an 8-pixel board, valid values for pixel are 0-7. Valid
    /// values for red, green and blue are 0-255. Valid values for brightness
    /// are 0.0-1.0, which is converted to a 5-bit value.
    pub fn set_pixel_rgbb(&mut self, pixel: usize, red: u8, green: u8, blue: u8, brightness: f32) {
        if let Some(pixel) = self.pixels.get_mut(pixel) {
            pixel.red = red;
            pixel.green = green;
            pixel.blue = blue;
            pixel.brightness = (31.0 * if brightness > 1.0 {
                1.0
            } else if brightness < 0.0 {
                0.0
            } else {
                brightness
            }) as u8;
        }
    }

    /// Sets the brightness value for a single pixel in the local buffer.
    ///
    /// For an 8-pixel board, valid values for pixel are 0-7. Valid
    /// values for brightness are 0.0-1.0, which is converted to a
    /// 5-bit value.
    pub fn set_pixel_brightness(&mut self, pixel: usize, brightness: f32) {
        if let Some(pixel) = self.pixels.get_mut(pixel) {
            pixel.brightness = (31.0 * if brightness > 1.0 {
                1.0
            } else if brightness < 0.0 {
                0.0
            } else {
                brightness
            }) as u8;
        }
    }

    /// Sets the red, green and blue values for all pixels in the local buffer.
    ///
    /// Valid values for red, green and blue are 0-255.
    pub fn set_all_pixels(&mut self, red: u8, green: u8, blue: u8) {
        for pixel in &mut self.pixels {
            pixel.red = red;
            pixel.green = green;
            pixel.blue = blue;
        }
    }

    /// Sets the red, green, blue and brightness values for all pixels in the
    /// local buffer.
    ///
    /// Valid values for red, green and blue are 0-255. Valid values for
    /// brightness are 0.0-1.0, which is converted to a 5-bit value.
    pub fn set_all_pixels_rgbb(&mut self, red: u8, green: u8, blue: u8, brightness: f32) {
        let brightness: u8 = (31.0 * if brightness > 1.0 {
            1.0
        } else if brightness < 0.0 {
            0.0
        } else {
            brightness
        }) as u8;
        for pixel in &mut self.pixels {
            pixel.red = red;
            pixel.green = green;
            pixel.blue = blue;
            pixel.brightness = brightness;
        }
    }

    /// Sets the brightness value for all pixels in the local buffer.
    ///
    /// Valid values for brightness are 0.0-1.0, which is converted to a 5-bit
    /// value.
    pub fn set_all_pixels_brightness(&mut self, brightness: f32) {
        let brightness: u8 = (31.0 * if brightness > 1.0 {
            1.0
        } else if brightness < 0.0 {
            0.0
        } else {
            brightness
        }) as u8;
        for pixel in &mut self.pixels {
            pixel.brightness = brightness;
        }
    }

    /// Sets the red, green and blue values to 0 for all pixels in the local
    /// buffer.
    pub fn clear(&mut self) {
        self.set_all_pixels(0, 0, 0);
    }

    /// Sends the contents of the local buffer to the pixels, updating their
    /// LED colors and brightness.
    pub fn show(&self) {
        // Start frame (32x0)
        self.write_byte(0);
        self.write_byte(0);
        self.write_byte(0);
        self.write_byte(0);

        // LED frames
        for pixel in &self.pixels {
            self.write_byte(0b11100000 | pixel.brightness); // 3-bit header + 5-bit brightness
            self.write_byte(pixel.blue);
            self.write_byte(pixel.green);
            self.write_byte(pixel.red);
        }

        // We send another start frame immediately after our end frame, because
        // the SK9822 clone won't update the pixels until it receives the next
        // start frame. We still start show() with a start frame, basically
        // sending it twice, in case the user connects a Blinkt! while the
        // code is already running. This workaround is compatible with both
        // the original APA102 and the SK9822 clone.
        self.gpio.write(self.pin_data, Level::Low);
        for _ in 0..32 + self.endframe_pulses {
            self.gpio.write(self.pin_clock, Level::High);
            self.gpio.write(self.pin_clock, Level::Low);
        }
    }

    fn write_byte(&self, byte: u8) {
        for n in 0..8 {
            if (byte & (1 << (7 - n))) > 0 {
                self.gpio.write(self.pin_data, Level::High);
            } else {
                self.gpio.write(self.pin_data, Level::Low);
            }

            self.gpio.write(self.pin_clock, Level::High);
            self.gpio.write(self.pin_clock, Level::Low);
        }
    }
}

impl Drop for Blinkt {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[test]
fn test_new() {
    let mut blinkt = match Blinkt::new() {
        // GPIO errors are acceptable, since they're likely caused by outside
        // distro/filesystem issues.
        Err(Error::Gpio(_)) => return,
        Ok(blinkt) => blinkt,
    };

    blinkt.set_clear_on_drop(false);
}
