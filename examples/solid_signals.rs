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

    let running = Arc::new(AtomicBool::new(true));

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
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
