// Copyright (c) 2016-2022 Rene van der Meer
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

const DEFAULT_BRIGHTNESS: u8 = 7;

const IDX_BRIGHTNESS: usize = 0;
const IDX_BLUE: usize = 1;
const IDX_GREEN: usize = 2;
const IDX_RED: usize = 3;

/// A pixel on an LED strip or board.
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    value: [u8; 4], // Brightness, blue, green, red
}

impl Pixel {
    /// Returns a tuple containing the values for red, green and blue.
    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            self.value[IDX_RED],
            self.value[IDX_GREEN],
            self.value[IDX_BLUE],
        )
    }

    /// Sets the values for red, green and blue.
    ///
    /// `red`, `green` and `blue` are specified as 8-bit values between `0` (0%) and `255` (100%).
    #[inline]
    pub fn set_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.value[IDX_RED] = red;
        self.value[IDX_GREEN] = green;
        self.value[IDX_BLUE] = blue;
    }

    /// Returns a tuple containing the values for red, green, blue and brightness.
    #[inline]
    pub fn rgbb(&self) -> (u8, u8, u8, f32) {
        (
            self.value[IDX_RED],
            self.value[IDX_GREEN],
            self.value[IDX_BLUE],
            f32::from(0b0001_1111 & self.value[IDX_BRIGHTNESS]) / 31.0,
        )
    }

    /// Sets the values for red, green, blue and brightness.
    ///
    /// `red`, `green` and `blue` are specified as 8-bit values between `0` (0%) and `255` (100%).
    /// `brightness` is specified as a floating point value between `0.0` (0%) and `1.0` (100%), and is converted to a 5-bit value.
    #[inline]
    pub fn set_rgbb(&mut self, red: u8, green: u8, blue: u8, brightness: f32) {
        self.set_rgb(red, green, blue);
        self.set_brightness(brightness);
    }

    /// Returns the red value.
    #[inline]
    pub fn red(&self) -> u8 {
        self.value[IDX_RED]
    }

    /// Sets the red value.
    ///
    /// `red` is specified as an 8-bit value between `0` (0%) and `255` (100%).
    #[inline]
    pub fn set_red(&mut self, red: u8) {
        self.value[IDX_RED] = red;
    }

    /// Returns the green value.
    #[inline]
    pub fn green(&self) -> u8 {
        self.value[IDX_GREEN]
    }

    /// Sets the green value.
    ///
    /// `green` is specified as an 8-bit value between `0` (0%) and `255` (100%).
    #[inline]
    pub fn set_green(&mut self, green: u8) {
        self.value[IDX_GREEN] = green;
    }

    /// Returns the blue value.
    #[inline]
    pub fn blue(&self) -> u8 {
        self.value[IDX_BLUE]
    }

    /// Sets the blue value.
    ///
    /// `blue` is specified as an 8-bit value between `0` (0%) and `255` (100%).
    #[inline]
    pub fn set_blue(&mut self, blue: u8) {
        self.value[IDX_BLUE] = blue;
    }

    /// Returns the brightness value.
    #[inline]
    pub fn brightness(&self) -> f32 {
        f32::from(0b0001_1111 & self.value[IDX_BRIGHTNESS]) / 31.0
    }

    /// Sets the brightness value.
    ///
    /// `brightness` is specified as a floating point value between `0.0` (0%) and `1.0` (100%), and is converted to a 5-bit value.
    #[inline]
    pub fn set_brightness(&mut self, brightness: f32) {
        self.value[IDX_BRIGHTNESS] = 0b1110_0000 | ((31.0 * brightness.max(0.0).min(1.0)) as u8);
    }

    /// Sets the red, green and blue values to `0`.
    #[inline]
    pub fn clear(&mut self) {
        self.set_rgb(0, 0, 0);
    }

    #[inline]
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.value
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            value: [0b1110_0000 | DEFAULT_BRIGHTNESS, 0, 0, 0],
        }
    }
}
