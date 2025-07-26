//! USB peripheral driver.

use core::mem::{self, MaybeUninit};
use cortex_m::interrupt::{self, Mutex};
use usb_device::bus::{PollResult, UsbBusAllocator};
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result, UsbDirection, UsbError};

use crate::endpoint::{calculate_count_rx, Endpoint, EndpointStatus, NUM_ENDPOINTS};
use crate::endpoint_memory::EndpointMemoryAllocator;
use crate::registers::UsbRegisters;
use crate::UsbPeripheral;

/// USB peripheral driver for STM32 microcontrollers.
pub struct UsbBus<USB> {
    peripheral: USB,
    regs: Mutex<UsbRegisters<USB>>,
    endpoints: [Endpoint<USB>; NUM_ENDPOINTS],
    ep_allocator: EndpointMemoryAllocator<USB>,
    max_endpoint: usize,
}

impl<USB: UsbPeripheral> UsbBus<USB> {
    /// Constructs a new USB peripheral driver.
    pub fn new(peripheral: USB) -> UsbBusAllocator<Self> {
        USB::enable();

        let bus = UsbBus {
            peripheral,
            regs: Mutex::new(UsbRegisters::new()),
            ep_allocator: EndpointMemoryAllocator::new(),
            max_endpoint: 0,
            endpoints: {
                let mut endpoints: [MaybeUninit<Endpoint<USB>>; NUM_ENDPOINTS] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for i in 0..NUM_ENDPOINTS {
                    endpoints[i] = MaybeUninit::new(Endpoint::new(i as u8));
                }

                unsafe { mem::transmute::<_, [Endpoint<USB>; NUM_ENDPOINTS]>(endpoints) }
            },
        };

        UsbBusAllocator::new(bus)
    }

    pub fn free(self) -> USB {
        self.peripheral
    }

    /// Simulates a disconnect from the USB bus, causing the host to reset and re-enumerate the
    /// device.
    ///
    /// Mostly used for development. By calling this at the start of your program ensures that the
    /// host re-enumerates your device after a new program has been flashed.
    ///
    /// `disconnect` parameter is used to provide a custom disconnect function.
    /// This function will be called with USB peripheral powered down
    /// and interrupts disabled.
    /// It should perform disconnect in a platform-specific way.
    pub fn force_reenumeration<F: FnOnce()>(&self, disconnect: F) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            let pdwn = regs.cntr.read().pdwn().bit_is_set();
            regs.cntr.modify(|_, w| w.pdwn().set_bit());

            disconnect();

            regs.cntr.modify(|_, w| w.pdwn().bit(pdwn));
        });
    }

    /// Enter suspend low-power mode
    ///
    /// This should be used when in suspend mode if low-power mode needs to be entered during
    /// suspend (bus-powered device). Application should call this when it is ready to decrease
    /// power consumption to meet power consumption requirements of the USB suspend condition
    /// (e.g. disable system clocks or reduce their frequency). When wake up event is received
    /// low power mode will be automatically disabled.
    ///
    /// Will not enter low-power mode if not in suspend state. Returns `true` if entered.
    pub fn suspend_low_power_mode(&self) -> bool {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);
            if regs.cntr.read().fsusp().is_suspend() {
                regs.cntr.modify(|_, w| w.lpmode().set_bit());
                true
            } else {
                false
            }
        })
    }

    /// Set/clear remote wakeup bit
    ///
    /// Allows to modify CNTR.RESUME bit from device to perform remote wake up from suspend
    /// (e.g. on keyboard/mouse input). To perform remote wake up: when a condition is met
    /// during suspend mode, use this method with `true` to set the RESUME bit. Then, after
    /// waiting between 1-15 ms (see reference manual) call it again with `false` to clear
    /// the bit.
    ///
    /// This method will not set the bit if device is not suspended because performing remote
    /// wake up in other mode is invalid and host will most likely disable such a device.
    pub fn remote_wakeup(&self, resume: bool) {
        interrupt::free(|cs| {
            self.regs.borrow(cs)
                .cntr.modify(|r, w| w.resume().bit(resume && r.fsusp().is_suspend()));
        })
    }
}

impl<USB: UsbPeripheral> usb_device::bus::UsbBus for UsbBus<USB> {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress> {
        for index in ep_addr.map(|a| a.index()..a.index() + 1).unwrap_or(1..NUM_ENDPOINTS) {
            let ep = &mut self.endpoints[index];

            match ep.ep_type() {
                None => {
                    ep.set_ep_type(ep_type);
                }
                Some(t) if t != ep_type => {
                    continue;
                }
                _ => {}
            };

            match ep_dir {
                UsbDirection::Out if !ep.is_out_buf_set() => {
                    let (out_size, size_bits) = calculate_count_rx(max_packet_size as usize)?;

                    let buffer = self.ep_allocator.allocate_buffer(out_size)?;

                    ep.set_out_buf(buffer, size_bits);

                    return Ok(EndpointAddress::from_parts(index, ep_dir));
                }
                UsbDirection::In if !ep.is_in_buf_set() => {
                    let size = (max_packet_size as usize + 1) & !0x01;

                    let buffer = self.ep_allocator.allocate_buffer(size)?;

                    ep.set_in_buf(buffer);

                    return Ok(EndpointAddress::from_parts(index, ep_dir));
                }
                _ => {}
            }
        }

        Err(match ep_addr {
            Some(_) => UsbError::InvalidEndpoint,
            None => UsbError::EndpointOverflow,
        })
    }

    fn enable(&mut self) {
        let mut max = 0;
        for (index, ep) in self.endpoints.iter().enumerate() {
            if ep.is_out_buf_set() || ep.is_in_buf_set() {
                max = index;
            }
        }

        self.max_endpoint = max;

        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            regs.cntr.modify(|_, w| w.pdwn().clear_bit());

            USB::startup_delay();

            regs.btable.modify(|_, w| w.btable().bits(0));
            regs.cntr.modify(|_, w| {
                w.fres().clear_bit();
                w.resetm().set_bit();
                w.suspm().set_bit();
                w.wkupm().set_bit();
                w.ctrm().set_bit()
            });
            regs.istr.modify(|_, w| unsafe { w.bits(0) });

            if USB::DP_PULL_UP_FEATURE {
                regs.bcdr.modify(|_, w| w.dppu().set_bit());
            }
        });
    }

    fn reset(&self) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            regs.istr.modify(|_, w| unsafe { w.bits(0) });
            regs.daddr.modify(|_, w| w.ef().set_bit().add().bits(0));

            for ep in self.endpoints.iter() {
                ep.configure(cs);
            }
        });
    }

    fn set_device_address(&self, addr: u8) {
        interrupt::free(|cs| {
            self.regs.borrow(cs).daddr.modify(|_, w| w.add().bits(addr as u8));
        });
    }

    fn poll(&self) -> PollResult {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            let istr = regs.istr.read();

            if istr.wkup().bit_is_set() {
                // Interrupt flag bits are write-0-to-clear, other bits should be written as 1 to avoid
                // race conditions
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.wkup().clear_bit());

                // Required by datasheet
                regs.cntr.modify(|_, w| w.fsusp().clear_bit());

                PollResult::Resume
            } else if istr.reset().bit_is_set() {
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.reset().clear_bit());

                PollResult::Reset
            } else if istr.susp().bit_is_set() {
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.susp().clear_bit());

                PollResult::Suspend
            } else if istr.ctr().bit_is_set() {
                let mut ep_out = 0;
                let mut ep_in_complete = 0;
                let mut ep_setup = 0;
                let mut bit = 1;

                for ep in &self.endpoints[0..=self.max_endpoint] {
                    let v = ep.read_reg();

                    if v.ctr_rx().bit_is_set() {
                        ep_out |= bit;

                        if v.setup().bit_is_set() {
                            ep_setup |= bit;
                        }
                    }

                    if v.ctr_tx().bit_is_set() {
                        ep_in_complete |= bit;
                        ep.clear_ctr_tx(cs);
                    }

                    bit <<= 1;
                }

                PollResult::Data {
                    ep_out,
                    ep_in_complete,
                    ep_setup,
                }
            } else {
                PollResult::None
            }
        })
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize> {
        if !ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[ep_addr.index()].write(buf)
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize> {
        if !ep_addr.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[ep_addr.index()].read(buf)
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        interrupt::free(|cs| {
            if self.is_stalled(ep_addr) == stalled {
                return;
            }

            let ep = &self.endpoints[ep_addr.index()];

            match (stalled, ep_addr.direction()) {
                (true, UsbDirection::In) => ep.set_stat_tx(cs, EndpointStatus::Stall),
                (true, UsbDirection::Out) => ep.set_stat_rx(cs, EndpointStatus::Stall),
                (false, UsbDirection::In) => ep.set_stat_tx(cs, EndpointStatus::Nak),
                (false, UsbDirection::Out) => ep.set_stat_rx(cs, EndpointStatus::Valid),
            };
        });
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let ep = &self.endpoints[ep_addr.index()];
        let reg_v = ep.read_reg();

        let status = match ep_addr.direction() {
            UsbDirection::In => reg_v.stat_tx().bits(),
            UsbDirection::Out => reg_v.stat_rx().bits(),
        };

        status == (EndpointStatus::Stall as u8)
    }

    fn suspend(&self) {
        interrupt::free(|cs| {
            self.regs
                .borrow(cs)
                .cntr
                .modify(|_, w| w.fsusp().set_bit());
        });
    }

    fn resume(&self) {
        interrupt::free(|cs| {
            self.regs
                .borrow(cs)
                .cntr
                .modify(|_, w| w.fsusp().clear_bit().lpmode().clear_bit());
        });
    }
}
