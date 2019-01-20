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

// solid_signals.rs - Swaps all pixels on a Blinkt! board between red, green
// and blue in a loop, while handling any incoming SIGINT (Ctrl-C) and SIGTERM
// signals so the pixels can be cleared before the application exits.

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{mem, thread};

// The simple-signal crate is used to handle incoming signals.
use simple_signal::{self, Signal};

use blinkt::Blinkt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;

    // Clone running, so we can safely access it from within the signal handler.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], move |_| {
        r.store(false, Ordering::SeqCst);
    });

    let (red, green, blue) = (&mut 255, &mut 0, &mut 0);

    // Loop until running is set to false.
    while running.load(Ordering::SeqCst) {
        blinkt.set_all_pixels(*red, *green, *blue);
        blinkt.show()?;

        thread::sleep(Duration::from_millis(250));

        mem::swap(red, green);
        mem::swap(red, blue);
    }

    Ok(())

    // When the blinkt variable goes out of scope, all pixels are automatically cleared.
}
