[![crates.io](https://img.shields.io/crates/d/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
[![crates.io](https://img.shields.io/crates/v/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
![Build Status](https://github.com/stm32-rs/stm32-usbd/workflows/CI/badge.svg)

# `stm32-usbd`

> [usb-device](https://github.com/mvirkkunen/usb-device) implementation for STM32
microcontrollers.

This project is a successor to the [great work](https://github.com/mvirkkunen/stm32f103xx-usb)
started by [@mvirkkunen](https://github.com/mvirkkunen).

## Supported microcontrollers

* `STM32F042xx`
* `STM32F048xx`
* `STM32F072xx`
* `STM32F078xx`
* `STM32F103xx`
* `STM32F303xC`
* `STM32L0x2xx`
* `STM32L4x2xx`
* And others...

## Usage

This driver is intended for use through a device hal library.
Such hal library should implement `UsbPeripheral` for the corresponding USB peripheral object.
This trait declares all the peripheral properties that may vary from one device family to the other.

## Examples

Example applications may be found in the individual device HALs:

- [stm32f0xx-hal USB Serial](https://github.com/stm32-rs/stm32f0xx-hal/blob/master/examples/usb_serial.rs)
- [stm32f1xx-hal USB Serial](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/usb_serial.rs)
- [stm32f3xx-hal USB Serial](https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/usb_serial.rs)
- [stm32l0xx-hal USB Serial](https://github.com/stm32-rs/stm32l0xx-hal/blob/master/examples/usb_serial.rs)
- [stm32l4xx-hal USB Serial](https://github.com/stm32-rs/stm32l4xx-hal/blob/master/examples/usb_serial.rs)
