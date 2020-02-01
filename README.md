[![crates.io](https://img.shields.io/crates/d/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
[![crates.io](https://img.shields.io/crates/v/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
[![Build Status](https://travis-ci.org/stm32-rs/stm32-usbd.svg?branch=master)](https://travis-ci.org/stm32-rs/stm32-usbd)

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
Additionally, hal should pass `ram_access_1x16` of `ram_access_2x16` feature to the `stm32-usbd` library to
define endpoint memory access scheme:
* `ram_access_1x16` - for "1x16 bits/word" access scheme
* `ram_access_2x16` - for "2x16 bits/word" access scheme

## Examples

See the [stm32-usbd-examples](https://github.com/stm32-rs/stm32-usbd-examples) repo for different device-specific examples.

See the `hal` example for the reference hal implementation.
