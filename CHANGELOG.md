# Changelog

## 0.5.1 (TBD)

* Upgrade RPPAL dependency to 0.10.0.
* Remove quick-error dependency.

## 0.5.0 (November 16, 2018)

* Add support for Raspberry Pi 3 A+.
* Replace spidev dependency with RPPAL's SPI module.
* (Breaking change) Add Error::Spi to indicate an SPI error occurred.

## 0.4.0 (April 21, 2018)

* Add support for hardware SPI through Blinkt::with_spi().
* (Breaking change) Add a Result return value to show() to catch potential SPI or GPIO errors.
* Fix miscalculated number of end frame pulses for increased performance.
* Replace start frame and end frame loops with slices for increased performance.

## 0.3.0 (March 16, 2018)

* Add support for Raspberry Pi 3 B+.

## 0.2.0 (October 6, 2017)

* Update internal struct/enum names for RPPAL 0.2.0 upgrade.
* (Breaking change) Rename GPIOError to GpioError, and Error::GPIO to Error::Gpio.

## 0.1.2 (March 1, 2017)

* Move GPIO and System modules to external crate.
* Remove temporary blinkt variable binding in new().

## 0.1.1 (September 6, 2016)

* Add start frame to show().

## 0.1.0 (September 2, 2016)

* Initial release.
