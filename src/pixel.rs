// Copyright (c) 2016-2019 Rene van der Meer
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

/// A pixel on an LED strip or board.
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    value: [u8; 4], // Brightness, blue, green, red
}

impl Pixel {
    pub fn set_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.value[1] = blue;
        self.value[2] = green;
        self.value[3] = red;
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.value[0] = 0b1110_0000 | ((31.0 * brightness.max(0.0).min(1.0)) as u8);
    }

    pub fn set_rgbb(&mut self, red: u8, green: u8, blue: u8, brightness: f32) {
        self.set_rgb(red, green, blue);
        self.set_brightness(brightness);
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.value
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            value: [0b1110_0000 | DEFAULT_BRIGHTNESS, 0, 0, 0],
        }
    }
}
