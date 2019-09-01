//! USB peripheral driver.

use usb_device::{Result, UsbDirection, UsbError};
use usb_device::allocator::{UsbAllocator, EndpointConfig};
use usb_device::bus::PollResult;
use usb_device::endpoint::{EndpointDescriptor, EndpointAddress};
use cortex_m::interrupt;
use cortex_m::asm::delay;

use crate::target::{self, NUM_ENDPOINTS, EP_MEM_SIZE, UsbPins, UsbAccessType, apb_usb_enable};
use crate::endpoint::{
    EndpointOut,
    EndpointIn,
    calculate_count_rx, 
    set_stalled,
    is_stalled};
use crate::endpoint_register::{EndpointRegister, EndpointStatus};
use crate::endpoint_buffer::{BufferDescriptor, EndpointBuffer};


/// USB peripheral driver for STM32 microcontrollers.
pub struct UsbBus {
    max_endpoint: usize,
}

impl UsbBus {
    /// Constructs a new USB peripheral driver.
    pub fn new<PINS: UsbPins>(regs: target::USB, pins: PINS) -> UsbAllocator<Self> {
        let _ = (regs, pins);

        apb_usb_enable();

        let bus = UsbBus {
            max_endpoint: 0,
        };

        UsbAllocator::new(bus)
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
    pub fn force_reenumeration<F: FnOnce()>(&mut self, disconnect: F) {
        interrupt::free(|_| {
            let pdwn = Self::regs().cntr.read().pdwn().bit_is_set();
            Self::regs().cntr.modify(|_, w| w.pdwn().set_bit());

            disconnect();

            Self::regs().cntr.modify(|_, w| w.pdwn().bit(pdwn));
        });
    }

    pub(crate) fn regs() -> &'static mut target::usb::RegisterBlock {
        unsafe { &mut *(target::USB::ptr() as *mut target::usb::RegisterBlock) }
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    type EndpointOut = EndpointOut;
    type EndpointIn = EndpointIn;
    type EndpointAllocator = EndpointAllocator;

    fn create_allocator(&mut self) -> EndpointAllocator {
        EndpointAllocator::new()
    }

    fn enable(&mut self) {
        /*let mut max = 0;
        for (index, ep) in self.endpoints.iter().enumerate() {
            if ep.is_out_buf_set() || ep.is_in_buf_set() {
                max = index;
            }
        }*/

        self.max_endpoint = NUM_ENDPOINTS - 1; // TODO

        Self::regs().cntr.modify(|_, w| w.pdwn().clear_bit());

        // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
        // at least that long.
        delay(72);

        Self::regs().btable.modify(|_, w| w.btable().bits(0));
        Self::regs().cntr.modify(|_, w| w
            .fres().clear_bit()
            .resetm().set_bit()
            .suspm().set_bit()
            .wkupm().set_bit()
            .ctrm().set_bit());
        Self::regs().istr.modify(|_, w| unsafe { w.bits(0) });

        #[cfg(feature = "dp_pull_up_support")]
        Self::regs().bcdr.modify(|_, w| w.dppu().set_bit());
    }

    fn reset(&mut self) {
        Self::regs().istr.modify(|_, w| unsafe { w.bits(0) });
        Self::regs().daddr.modify(|_, w| w.ef().set_bit().add().bits(0));

        for i in 1..=self.max_endpoint {
            let mut epr = EndpointRegister::get(i as u8);
            epr.set_stat_rx(EndpointStatus::Disabled);
            epr.set_stat_tx(EndpointStatus::Disabled);
        }
    }

    fn set_device_address(&mut self, addr: u8) {
        Self::regs().daddr.modify(|_, w| w.add().bits(addr as u8));
    }

    fn poll(&mut self) -> PollResult {
        let istr = Self::regs().istr.read();

        if istr.wkup().bit_is_set() {
            // Interrupt flag bits are write-0-to-clear, other bits should be written as 1 to avoid
            // race conditions
            Self::regs().istr.write(|w| unsafe { w.bits(0xffff) }.wkup().clear_bit() );

            // Required by datasheet
            Self::regs().cntr.modify(|_, w| w.fsusp().clear_bit());

            PollResult::Resume
        } else if istr.reset().bit_is_set() {
            Self::regs().istr.write(|w| unsafe { w.bits(0xffff) }.reset().clear_bit() );

            PollResult::Reset
        } else if istr.susp().bit_is_set() {
            Self::regs().istr.write(|w| unsafe { w.bits(0xffff) }.susp().clear_bit() );

            PollResult::Suspend
        } else if istr.ctr().bit_is_set() {
            let mut ep_out = 0;
            let mut ep_in_complete = 0;
            let mut ep_setup = 0;

            for ep in 0..=self.max_endpoint {
                let mut epr = EndpointRegister::get(ep as u8);
                let epr_v = epr.read();

                if epr_v.ctr_rx().bit_is_set() {
                    ep_out |= 1 << ep;

                    if epr_v.setup().bit_is_set() {
                        ep_setup |= 1 << ep;
                    }
                }

                if epr_v.ctr_tx().bit_is_set() {
                    ep_in_complete |= 1 << ep;
                    epr.clear_ctr_tx();
                }
            }

            PollResult::Data { ep_out, ep_in_complete, ep_setup }
        } else {
            PollResult::None
        }
    }

    fn suspend(&mut self) {
        Self::regs().cntr.modify(|_, w| w
            .fsusp().set_bit()
            .lpmode().set_bit());
    }

    fn resume(&mut self) {
        Self::regs().cntr.modify(|_, w| w
            .fsusp().clear_bit()
            .lpmode().clear_bit());
    }

    fn set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        set_stalled(ep_addr, stalled);
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        is_stalled(ep_addr)
    }
}

pub struct EndpointAllocator {
    next_endpoint_number: u8,
    next_free_offset: usize,
    out_taken: u16,
    in_taken: u16,
}

impl EndpointAllocator {
    fn new() -> Self {
        EndpointAllocator {
            next_endpoint_number: 1,
            next_free_offset: NUM_ENDPOINTS * 8,
            out_taken: 0,
            in_taken: 0,
        }
    }

    fn alloc_ep(&mut self, direction: UsbDirection, config: &EndpointConfig) 
        -> Result<(EndpointDescriptor, EndpointBuffer)>
    {
        // TODO: Use pair_of information

        let number = config.number.unwrap_or(self.next_endpoint_number);
        if number as usize >= NUM_ENDPOINTS {
            return Err(UsbError::EndpointOverflow);
        }
        
        let buffer = match direction {
            UsbDirection::Out => {
                if self.out_taken & (1 << number) != 0 {
                    return Err(UsbError::InvalidEndpoint);
                }

                self.out_taken |= 1 << number;

                let (out_size, size_bits) = calculate_count_rx(config.max_packet_size as usize)?;

                let buffer = self.alloc_buffer(out_size)?;

                let descr = BufferDescriptor::get(number);
                descr.addr_rx.set(buffer.offset() as UsbAccessType);
                descr.count_rx.set(size_bits as UsbAccessType);

                buffer
            },
            UsbDirection::In => {
                if self.in_taken & (1 << number) != 0 {
                    return Err(UsbError::InvalidEndpoint);
                }

                self.in_taken |= 1 << number;

                let size = (config.max_packet_size as usize + 1) & !0x01;

                let buffer = self.alloc_buffer(size)?;

                let descr = BufferDescriptor::get(number);
                descr.addr_tx.set(buffer.offset() as UsbAccessType);
                descr.count_tx.set(0);

                buffer
            },
        };

        if config.number.is_none() {
            self.next_endpoint_number += 1;
        }

        let descriptor = EndpointDescriptor {
            address: EndpointAddress::from_parts(number, direction),
            ep_type: config.ep_type,
            max_packet_size: config.max_packet_size,
            interval: config.interval,
        };

        Ok((descriptor, buffer))
    }

    fn alloc_buffer(&mut self, size: usize) -> Result<EndpointBuffer> {
        assert!(size & 1 == 0);
        assert!(size < EP_MEM_SIZE);

        let offset = self.next_free_offset;
        if offset as usize + size > EP_MEM_SIZE {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        self.next_free_offset += size;

        Ok(EndpointBuffer::new(offset, size))
    }
}

impl usb_device::bus::EndpointAllocator<UsbBus> for EndpointAllocator {
    fn alloc_out(&mut self, config: &EndpointConfig) -> Result<EndpointOut> {
        let (descriptor, buf) = self.alloc_ep(UsbDirection::Out, config)?;

        Ok(EndpointOut { descriptor, buf })
    }
    
    fn alloc_in(&mut self, config: &EndpointConfig) -> Result<EndpointIn> {
        let (descriptor, buf) = self.alloc_ep(UsbDirection::In, config)?;

        Ok(EndpointIn { descriptor, buf })
    }
}
