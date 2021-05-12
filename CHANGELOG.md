# Changelog

## 0.6.0 (May 12, 2021)

* (Breaking change) Transition to Rust 2018, requiring rustc v1.45.0 or newer to compile the library.
* Add new badge to `README.md`, indicating the required minimum rustc version.
* Upgrade `rppal` dependency to 0.12.0.
* Add support for Raspberry Pi CM3+, CM4, 4B, 400.
* Remove `quick-error` dependency.
* Add `Pixel` to public interface, which represents a pixel on an LED strip or board.
* Add `IterMut`, which implements `Iterator` for a `&mut Pixel` slice.
* Implement `IntoIterator` for `&mut Blinkt`, which returns an `IterMut` mutable iterator over all `Pixel`s stored in `Blinkt`.
* Add `Blinkt::iter_mut()`, which returns an `IterMut` mutable iterator over all `Pixel`s stored in `Blinkt`.
* Add `Blinkt::clear_on_drop()`, which returns the current value of `clear_on_drop`.
* (Breaking change) Remove `Blinkt::cleanup()`. When `Blinkt` goes out of scope, any changed pin states are automatically reset. If `clear_on_drop` is set to `true`, all pixels will also be cleared.
* Implement `Send` for `Blinkt`.

## 0.5.0 (November 16, 2018)

* Add support for Raspberry Pi 3 A+.
* Replace `spidev` dependency with `rppal`'s SPI module.
* (Breaking change) Add `Error::Spi` to indicate an SPI error occurred.

## 0.4.0 (April 21, 2018)

* Add support for hardware SPI through `Blinkt::with_spi()`.
* (Breaking change) Add a `Result` return value to `Blinkt::show()` to catch potential SPI or GPIO errors.
* Fix miscalculated number of end frame pulses for increased performance.
* Replace start frame and end frame loops with slices for increased performance.

## 0.3.0 (March 16, 2018)

* Add support for Raspberry Pi 3 B+.

## 0.2.0 (October 6, 2017)

* Update internal struct/enum names for `rppal` 0.2.0 upgrade.
* (Breaking change) Rename `GPIOError` to `GpioError`, and `Error::GPIO` to `Error::Gpio`.

## 0.1.2 (March 1, 2017)

* Move `GPIO` and `System` modules to external crate.
* Remove temporary `blinkt` variable binding in `new()`.

## 0.1.1 (September 6, 2016)

* Add start frame to `show()`.

## 0.1.0 (September 2, 2016)

* Initial release.
