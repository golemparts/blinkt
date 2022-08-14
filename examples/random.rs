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
