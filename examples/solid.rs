// solid.rs - Swaps all pixels on a Blinkt! board between red, green and blue
// in a loop.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without clearing the pixels. Check out the solid_signals.rs
// example to learn how to properly handle incoming signals to prevent an
// abnormal termination.

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
