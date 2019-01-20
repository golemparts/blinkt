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

// random.rs - Sets each pixel on a Blinkt! board to a random RGB value in a loop.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without clearing the pixels. Check out the solid_signals.rs
// example to learn how to properly handle incoming signals to prevent an
// abnormal termination.

use std::error::Error;
use std::thread;
use std::time::Duration;

use rand::{self, Rng};

use blinkt::Blinkt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    let mut rng = rand::thread_rng();

    blinkt.set_all_pixels_brightness(0.1);

    loop {
        // Iterate over all pixels, setting red, green and blue to random values.
        for pixel in &mut blinkt {
            pixel.set_rgb(rng.gen(), rng.gen(), rng.gen());
        }

        // Send the new color values to the Blinkt! board.
        blinkt.show()?;

        thread::sleep(Duration::from_millis(100));
    }
}
