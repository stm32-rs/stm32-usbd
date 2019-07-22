use core::marker::PhantomData;
use core::mem;
use usb_device::{Result, UsbDirection, UsbError};
use usb_device::bus::{UsbBusAllocator, PollResult};
use usb_device::endpoint::{EndpointType, EndpointAddress};
use cortex_m::interrupt::{self, Mutex};

use crate::target::{USB, apb_usb_enable, delay, NUM_ENDPOINTS, UsbRegisters, UsbPins};
use crate::endpoint::{Endpoint, EndpointStatus, calculate_count_rx};
use crate::endpoint_memory::EndpointMemoryAllocator;


/// USB peripheral driver for STM32 microcontrollers.
pub struct UsbBus<PINS> {
    regs: Mutex<UsbRegisters>,
    endpoints: [Endpoint; NUM_ENDPOINTS],
    ep_allocator: EndpointMemoryAllocator,
    max_endpoint: usize,
    pins: PhantomData<PINS>,
}

impl<PINS: UsbPins+Sync> UsbBus<PINS> {
    /// Constructs a new USB peripheral driver.
    pub fn new(regs: USB, _pins: PINS) -> UsbBusAllocator<Self> {
        apb_usb_enable();

        let bus = UsbBus {
            regs: Mutex::new(UsbRegisters::new(regs)),
            ep_allocator: EndpointMemoryAllocator::new(),
            max_endpoint: 0,
            endpoints: unsafe {
                let mut endpoints: [Endpoint; NUM_ENDPOINTS] = mem::uninitialized();

                for i in 0..NUM_ENDPOINTS {
                    endpoints[i] = Endpoint::new(i as u8);
                }

                endpoints
            },
            pins: PhantomData,
        };

        UsbBusAllocator::new(bus)
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
    pub fn force_reenumeration<F: FnOnce()>(&mut self, disconnect: F)
    {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            let pdwn = regs.cntr.read().pdwn().bit_is_set();
            regs.cntr.modify(|_, w| w.pdwn().set_bit());

            disconnect();

            regs.cntr.modify(|_, w| w.pdwn().bit(pdwn));
        });
    }
}

impl<PINS: Send+Sync> usb_device::bus::UsbBus for UsbBus<PINS> {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8) -> Result<EndpointAddress>
    {
        for index in ep_addr.map(|a| a.index()..a.index()+1).unwrap_or(1..NUM_ENDPOINTS) {
            let ep = &mut self.endpoints[index];

            match ep.ep_type() {
                None => { ep.set_ep_type(ep_type); },
                Some(t) if t != ep_type => { continue; },
                _ => { },
            };

            match ep_dir {
                UsbDirection::Out if !ep.is_out_buf_set() => {
                    let (out_size, size_bits) = calculate_count_rx(max_packet_size as usize)?;

                    let buffer = self.ep_allocator.allocate_buffer(out_size)?;

                    ep.set_out_buf(buffer, size_bits);

                    return Ok(EndpointAddress::from_parts(index, ep_dir));
                },
                UsbDirection::In if !ep.is_in_buf_set() => {
                    let size = (max_packet_size as usize + 1) & !0x01;

                    let buffer = self.ep_allocator.allocate_buffer(size)?;

                    ep.set_in_buf(buffer);

                    return Ok(EndpointAddress::from_parts(index, ep_dir));
                }
                _ => { }
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

            // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
            // at least that long.
            delay(72);

            regs.btable.modify(|_, w| w.btable().bits(0));
            regs.cntr.modify(|_, w| w
                .fres().clear_bit()
                .resetm().set_bit()
                .suspm().set_bit()
                .wkupm().set_bit()
                .ctrm().set_bit());
            regs.istr.modify(|_, w| unsafe { w.bits(0) });

            #[cfg(feature = "dp_pull_up_support")]
            regs.bcdr.modify(|_, w| w.dppu().set_bit());
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
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.wkup().clear_bit() );

                // Required by datasheet
                regs.cntr.modify(|_, w| w.fsusp().clear_bit());

                PollResult::Resume
            } else if istr.reset().bit_is_set() {
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.reset().clear_bit() );

                PollResult::Reset
            } else if istr.susp().bit_is_set() {
                regs.istr.write(|w| unsafe { w.bits(0xffff) }.susp().clear_bit() );

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

                        interrupt::free(|cs| {
                            ep.clear_ctr_tx(cs);
                        });
                    }

                    bit <<= 1;
                }

                PollResult::Data { ep_out, ep_in_complete, ep_setup }
            } else {
                PollResult::None
            }
        })
    }

    fn can_write(&self, ep_addr: EndpointAddress) -> Result<bool> {
        if !ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }

        Ok(self.endpoints[ep_addr.index()].can_write())
    }

    fn write_complete(&self, ep_addr: EndpointAddress) -> Result<bool> {
        if !ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }

        Ok(self.endpoints[ep_addr.index()].write_complete())
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize> {
        if !ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[ep_addr.index()].write(buf)
    }

    fn available_read(&self, ep_addr: EndpointAddress) -> Result<usize> {
        if !ep_addr.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[ep_addr.index()].available_read()
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
                return
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
             self.regs.borrow(cs).cntr.modify(|_, w| w
                .fsusp().set_bit()
                .lpmode().set_bit());
        });
    }

    fn resume(&self) {
        interrupt::free(|cs| {
            self.regs.borrow(cs).cntr.modify(|_, w| w
                .fsusp().clear_bit()
                .lpmode().clear_bit());
        });
    }
}
