//! USB peripheral driver.

use usb_device::endpoint::{EndpointAddress, EndpointConfig, EndpointType};
use usb_device::usbcore::{self, PollResult};
use usb_device::{Result, UsbDirection, UsbError};

use crate::endpoint::{calculate_count_rx, EndpointPair, UsbEndpointIn, UsbEndpointOut, NUM_ENDPOINTS};
use crate::endpoint_memory::{EndpointMemoryAllocator, UsbAccessType};
use crate::registers::UsbRegisters;
use crate::UsbPeripheral;

/// USB peripheral driver for STM32 microcontrollers.
pub struct UsbCore<USB> {
    peripheral: USB,
    regs: UsbRegisters<USB>,
    endpoint_cap: usize,
}

impl<USB: UsbPeripheral> UsbCore<USB> {
    /// Constructs a new USB peripheral driver.
    pub fn new(peripheral: USB) -> Self {
        USB::enable();

        UsbCore {
            peripheral,
            regs: UsbRegisters::new(),
            endpoint_cap: 0,
        }
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
        let pdwn = self.regs.cntr.read().pdwn().bit_is_set();
        self.regs.cntr.modify(|_, w| w.pdwn().set_bit());

        disconnect();

        self.regs.cntr.modify(|_, w| w.pdwn().bit(pdwn));
    }

    fn configure_endpoints(&mut self, alloc: &UsbEndpointAllocator) -> Result<usize> {
        let mut max_endpoint = 0;

        let mem = USB::EP_MEMORY as *mut UsbAccessType;

        unsafe {
            core::ptr::write_bytes(mem, 0, USB::EP_MEMORY_SIZE);
        }

        let mut memory_alloc = EndpointMemoryAllocator::<USB>::new();

        for i in 0..NUM_ENDPOINTS {
            let ep = EndpointPair::<USB>::new(i as u8);

            let ep_in = &alloc.ep_in[i];
            if ep_in.iface.is_some() {
                let offset = memory_alloc.allocate_buffer(ep_in.max_packet_size as usize)?;

                ep.descr().addr_tx.set(offset as UsbAccessType);
                ep.descr().count_tx.set(0);

                max_endpoint = i + 1;
            }

            let ep_out = &alloc.ep_out[i];
            if ep_out.iface.is_some() {
                let (size, size_bits) = calculate_count_rx(ep_out.max_packet_size as usize)?;
                let offset = memory_alloc.allocate_buffer(size as usize)?;

                ep.descr().addr_rx.set(offset as UsbAccessType);
                ep.descr().count_rx.set(size_bits as UsbAccessType);

                max_endpoint = i + 1;
            }
        }

        Ok(max_endpoint)
    }

    /*pub fn debug_dump(&self) {
        for i in 0..=2 {
            let ep = EndpointPair::<USB>::new(i as u8);

            rprintln!("EP {}:", i);
            rprintln!("  addr_tx {} count_tx {:04x} addr_rx {} count_rx {:04x}",
                ep.descr().addr_tx.get(),
                ep.descr().count_tx.get(),
                ep.descr().addr_rx.get(),
                ep.descr().count_rx.get());

            let v = ep.reg().read();

            rprintln!("  ctr_rx {} stat_rx {} ep_type {} ctr_tx {} stat_tx {} ea {}",
                v.ctr_rx().bit_is_set(),
                v.stat_rx().bits(),
                v.ep_type().bits(),
                v.ctr_tx().bit_is_set(),
                v.stat_tx().bits(),
                v.ea().bits());
        }
    }

    pub fn memory_dump(&self) {
        let mem = USB::EP_MEMORY as *mut UsbAccessType;

        for i in 0..64 {
            let p = unsafe { mem.offset(i as isize) };

            rprintln!("{:?} {:04x}", p, unsafe { core::ptr::read(p) } & 0xffff);
            //rprintln!("{:04x} {:04x}", i, unsafe { core::ptr::read(mem.offset(i as isize)) } & 0xffff);
        }
    }*/
}

impl<USB: UsbPeripheral> usbcore::UsbCore for UsbCore<USB> {
    type EndpointAllocator = UsbEndpointAllocator;

    type EndpointOut = UsbEndpointOut<USB>;

    type EndpointIn = UsbEndpointIn<USB>;

    fn create_allocator(&mut self) -> Self::EndpointAllocator {
        Default::default()
    }

    fn enable(&mut self, ep_alloc: UsbEndpointAllocator) -> Result<()> {
        self.regs.cntr.modify(|_, w| w.pdwn().clear_bit());

        USB::startup_delay();

        self.regs.btable.modify(|_, w| w.btable().bits(0));
        self.regs.cntr.modify(|_, w| {
            w.fres().clear_bit();
            w.resetm().set_bit();
            w.suspm().set_bit();
            w.wkupm().set_bit();
            w.ctrm().set_bit()
        });
        self.regs.istr.modify(|_, w| unsafe { w.bits(0) });

        if USB::DP_PULL_UP_FEATURE {
            self.regs.bcdr.modify(|_, w| w.dppu().set_bit());
        }

        self.endpoint_cap = self.configure_endpoints(&ep_alloc)?;

        Ok(())
    }

    fn reset(&mut self) {
        self.regs.istr.modify(|_, w| unsafe { w.bits(0) });
        self.regs.daddr.modify(|_, w| w.ef().set_bit().add().bits(0));

        for index in 1..self.endpoint_cap {
            let ep = EndpointPair::<USB>::new(index as u8);

            ep.disable_out();
            ep.disable_in();
        }
    }

    fn set_device_address(&mut self, addr: u8) {
        self.regs.daddr.modify(|_, w| w.add().bits(addr as u8));
    }

    fn poll(&mut self) -> PollResult {
        let istr = self.regs.istr.read();

        if istr.wkup().bit_is_set() {
            // Interrupt flag bits are write-0-to-clear, other bits should be written as 1 to avoid
            // race conditions
            self.regs.istr.write(|w| unsafe { w.bits(0xffff) }.wkup().clear_bit());

            // Required by datasheet
            self.regs.cntr.modify(|_, w| w.fsusp().clear_bit());

            PollResult::Resume
        } else if istr.reset().bit_is_set() {
            self.regs.istr.write(|w| unsafe { w.bits(0xffff) }.reset().clear_bit());

            PollResult::Reset
        } else if istr.susp().bit_is_set() {
            self.regs.istr.write(|w| unsafe { w.bits(0xffff) }.susp().clear_bit());

            PollResult::Suspend
        } else if istr.ctr().bit_is_set() {
            let mut ep_out = 0;
            let mut ep_in_complete = 0;
            let mut bit = 1;

            for index in 0..self.endpoint_cap {
                let ep = EndpointPair::<USB>::new(index as u8);

                let v = ep.reg().read();

                if v.ctr_rx().bit_is_set() {
                    ep_out |= bit;
                }

                if v.ctr_tx().bit_is_set() {
                    ep_in_complete |= bit;
                    ep.clear_ctr_tx();
                }

                bit <<= 1;
            }

            /*if ep_out != 0 {
                self.memory_dump();
            }*/

            PollResult::Data { ep_out, ep_in_complete }
        } else {
            PollResult::None
        }
    }

    fn set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let ep = EndpointPair::<USB>::new(ep_addr.number());

        match ep_addr.direction() {
            UsbDirection::Out => ep.set_out_stalled(stalled),
            UsbDirection::In => ep.set_in_stalled(stalled),
        }
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let ep = EndpointPair::<USB>::new(ep_addr.number());

        match ep_addr.direction() {
            UsbDirection::Out => ep.is_out_stalled(),
            UsbDirection::In => ep.is_in_stalled(),
        }
    }

    fn suspend(&mut self) {
        self.regs.cntr.modify(|_, w| w.fsusp().set_bit().lpmode().set_bit());
    }

    fn resume(&mut self) {
        self.regs.cntr.modify(|_, w| w.fsusp().clear_bit().lpmode().clear_bit());
    }
}

#[derive(Default)]
struct EpConfig {
    iface: Option<u8>,
    max_packet_size: u16,
}

#[derive(Default)]
pub struct UsbEndpointAllocator {
    iface: u8,
    ep_type_in_alt: [Option<EndpointType>; NUM_ENDPOINTS],
    ep_out: [EpConfig; NUM_ENDPOINTS],
    ep_in: [EpConfig; NUM_ENDPOINTS],
}

impl UsbEndpointAllocator {
    fn alloc(&mut self, dir: UsbDirection, config: &EndpointConfig) -> Result<(u8, u16)> {
        let mut eps = if dir == UsbDirection::Out {
            &mut self.ep_out
        } else {
            &mut self.ep_in
        };

        let range = match config.fixed_address() {
            Some(addr) => {
                if addr.direction() != dir {
                    return Err(UsbError::EndpointUnavailable);
                }

                let i = addr.number() as usize;

                i..(i + 1)
            }
            None => (1..NUM_ENDPOINTS),
        };

        for i in range {
            let iface = self.iface;

            if eps[i].iface.map(|i| i != iface).unwrap_or(false)
                || self.ep_type_in_alt[i].map(|t| t != config.ep_type()).unwrap_or(false)
            {
                continue;
            }

            eps[i].iface = Some(self.iface);
            self.ep_type_in_alt[i] = Some(config.ep_type());

            // Round to the nearest even size
            let size = (config.max_packet_size() + 1) & !1;

            if size > eps[i].max_packet_size {
                eps[i].max_packet_size = size;
            }

            return Ok((i as u8, size));
        }

        return Err(if config.fixed_address().is_some() {
            UsbError::EndpointUnavailable
        } else {
            UsbError::EndpointOverflow
        });
    }
}

impl<USB: UsbPeripheral> usbcore::UsbEndpointAllocator<UsbCore<USB>> for UsbEndpointAllocator {
    fn alloc_out(&mut self, config: &EndpointConfig) -> Result<UsbEndpointOut<USB>> {
        self.alloc(UsbDirection::Out, config)
            .map(|(index, buf_size_bytes)| UsbEndpointOut::new(index, buf_size_bytes))
    }

    fn alloc_in(&mut self, config: &EndpointConfig) -> Result<UsbEndpointIn<USB>> {
        self.alloc(UsbDirection::In, config)
            .map(|(index, buf_size_bytes)| UsbEndpointIn::new(index, buf_size_bytes))
    }

    fn begin_interface(&mut self) -> Result<()> {
        self.iface += 1;

        // Why can't type inference figure this out...?
        usbcore::UsbEndpointAllocator::<UsbCore<USB>>::next_alt_setting(self)
        //self.next_alt_setting()
    }

    fn next_alt_setting(&mut self) -> Result<()> {
        for i in 0..NUM_ENDPOINTS {
            self.ep_type_in_alt[i] = None;
        }

        Ok(())
    }
}
