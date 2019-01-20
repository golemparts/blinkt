# Examples

`random.rs` - Sets each pixel on a Blinkt! board to a random RGB value in a loop.

`solid.rs` - Swaps all pixels on a Blinkt! board between red, green and blue in a loop.

`solid_signals.rs` - Swaps all pixels on a Blinkt! board between red, green and blue in a loop, while handling any incoming `SIGINT` (<kbd>Ctrl</kbd> + <kbd>C</kbd>) and `SIGTERM` signals so the pixels can be cleared before the application exits.
