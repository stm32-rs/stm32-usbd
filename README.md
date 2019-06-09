[![crates.io](https://img.shields.io/crates/d/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
[![crates.io](https://img.shields.io/crates/v/stm32-usbd.svg)](https://crates.io/crates/stm32-usbd)
[![Build Status](https://travis-ci.org/stm32-rs/stm32-usbd.svg?branch=master)](https://travis-ci.org/stm32-rs/stm32-usbd)

# `stm32-usbd`

> [usb-device](https://github.com/mvirkkunen/usb-device) implementation for STM32
microcontrollers.

This repository is a fork for the [mvirkkunen/stm32f103xx-usb](https://github.com/mvirkkunen/stm32f103xx-usb) crate.

## Supported microcontrollers

* `STM32F042xx`
* `STM32F048xx`
* `STM32F072xx`
* `STM32F078xx`
* `STM32F103xx`
* `STM32F303xC`
* `STM32L4x2xx`
* And others...

## Feature flags

To use this crate you need to set proper feature flags.
All the feature flags are listed in `Cargo.toml` under different sections.

If your target MCU matches one of the feature flags listed under the
`Known devices` section of `Cargo.toml`, you can use just that feature flag. For example:

```toml
[dependencies]
stm32-usbd = { version = "0.2", features = ["stm32f103xx"] }
```

For other cases, you have to figure out different properties
of your MCU and USB peripheral implemented in it.
Each property has a corresponding feature flag that you should
set to indicate that property.

Device family:
* `stm32f0`
* `stm32f1`
* `stm32f3`
* `stm32l4`

Size of dedicated packet buffer memory SRAM:
* `ram_size_512`
* `ram_size_1024`

Dedicated packet buffer memory SRAM access scheme:
* `ram_access_1x16` - for "1x16 bits/word" access scheme
* `ram_access_2x16` - for "2x16 bits/word" access scheme

USB peripheral features:
* `lpm_support` - USB 2.0 Link Power Management (LPM) support
* `bcd_support` - Battery Charging Detection (BCD) support
* `dp_pull_up_support` - Embedded pull-up resistor on USB_DP line

Various hacks:
* `ram_addr_40006c00` if dedicated SRAM address is equal to `0x4000_6c00`
instead of `0x4000_6000`

```toml
[dependencies]
# An example feature set for STM32F303CB MCU
stm32-usbd = { version = "0.2", features = ["stm32f3", "ram_size_512", "ram_access_1x16"] }
stm32f3xx-hal = { version = "0.1.4", features = ["rt", "stm32f3xx-hal/stm32f303"] }
```

Note that you also need to set the device feature for `stm32*-hal` crate.

## Examples

See the `examples` directory for an example of a custom class and a minimalistic USB serial port device.

See the [stm32-usbd-examples](https://github.com/stm32-rs/stm32-usbd-examples) repo for different device-specific examples.
