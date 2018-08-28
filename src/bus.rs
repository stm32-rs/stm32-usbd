use core::cell::{Cell, RefCell};
use usb_device::{Result, UsbError};
use usb_device::bus::{EndpointType, EndpointPair, PollResult};
use cortex_m::asm::delay;
use stm32f103xx::{USB, usb};
use stm32f103xx_hal::rcc::APB1;
use regs::{NUM_ENDPOINTS, PacketMemory, EpReg, EndpointStatus, calculate_count_rx};

pub struct UsbBus {
    regs: USB,
    packet_mem: RefCell<PacketMemory>,
    max_endpoint: Cell<usize>,
    endpoints_taken: Cell<bool>,
}

impl UsbBus {
    pub fn usb(regs: USB, apb1: &mut APB1) -> UsbBus {
        // TODO: apb1.enr is not public, figure out how this should really interact with the HAL
        // crate
        let _ = apb1;
        ::cortex_m::interrupt::free(|_| {
            let dp = unsafe { ::stm32f103xx::Peripherals::steal() };
            dp.RCC.apb1enr.modify(|_, w| w.usben().enabled());
        });

        UsbBus {
            regs,
            packet_mem: RefCell::new(PacketMemory::new()),
            max_endpoint: Cell::new(0),
            endpoints_taken: Cell::new(false),
        }
    }

    pub fn endpoints(&self) -> Option<Endpoints> {
        if self.endpoints_taken.replace(true) {
            None
        } else {
            Some(Endpoints::new(self))
        }
    }

    fn ep_regs(&self) -> &'static [EpReg; NUM_ENDPOINTS] {
        return unsafe { &*(&self.regs.ep0r as *const usb::EP0R as *const EpReg as *const [EpReg; NUM_ENDPOINTS]) };
    }
}

impl ::usb_device::UsbBus for UsbBus {
    fn enable(&self) {
        self.regs.cntr.modify(|_, w| w.pdwn().clear_bit());

        // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
        // at least that long.
        delay(72);

        self.regs.btable.modify(|_, w| unsafe { w.btable().bits(0) });
        self.regs.cntr.modify(|_, w| w.fres().clear_bit());
        self.regs.istr.modify(|_, w| unsafe { w.bits(0) });
    }

    fn reset(&self) {
        self.regs.istr.modify(|_, w| unsafe { w.bits(0) });

        self.packet_mem.borrow_mut().reset();
        self.max_endpoint.set(0);

        self.regs.daddr.modify(|_, w| unsafe { w.ef().set_bit().add().bits(0) });
    }

    fn configure_ep(&self, ep_addr: u8, ep_type: EndpointType, max_packet_size: u16) -> Result<()> {
        let ep = (ep_addr & !0x80) as usize;
        let reg = &self.ep_regs()[ep];
        let mut pmem = self.packet_mem.borrow_mut();

        reg.configure(ep_type, ep_addr);

        if ep_addr & 0x80 == 0 {
            let (out_size, bits) = calculate_count_rx(max_packet_size as usize)?;

            let addr_rx = pmem.alloc(out_size)?;
            let bd = &pmem.descrs()[ep];

            bd.addr_rx.set(addr_rx);
            bd.count_rx.set(bits as usize);
            reg.set_stat_rx(EndpointStatus::Valid);
        } else {
            let addr_tx = pmem.alloc(max_packet_size as usize)?;
            let bd = &pmem.descrs()[ep];

            bd.addr_tx.set(addr_tx);
            bd.count_tx.set(0);
            reg.set_stat_tx(EndpointStatus::Nak);
        }

        if ep > self.max_endpoint.get() {
            self.max_endpoint.set(ep);
        }

        Ok(())
    }

    fn set_device_address(&self, addr: u8) {
        self.regs.daddr.modify(|_, w| unsafe { w.add().bits(addr as u8) });
    }

    fn poll(&self) -> PollResult {
        let istr = self.regs.istr.read();

        let mut res = PollResult::default();

        if istr.reset().bit_is_set() {
            res.reset = true;
            return res;
        }

        if istr.err().bit_is_set() {
            return res;
        }

        if istr.ctr().bit_is_set() {
            let mut bit = 1;

            for reg in &self.ep_regs()[0..=self.max_endpoint.get()] {
                let v = reg.read();

                if v.ctr_rx().bit_is_set() {
                    res.ep_out |= bit;

                    if bit == 1 && v.setup().bit_is_set() {
                        res.setup = true;
                    }
                }

                if v.ctr_tx().bit_is_set() {
                    res.ep_in_complete |= bit;

                    reg.clear_ctr_tx();
                }

                bit <<= 1;
            }
        }

        res
    }

    fn write(&self, ep_addr: u8, buf: &[u8]) -> Result<usize> {
        if ep_addr & 0x80 == 0 {
            return Err(UsbError::InvalidEndpoint);
        }

        let ep = ep_addr & !0x80;

        if ep as usize >= NUM_ENDPOINTS {
            return Err(UsbError::InvalidEndpoint);
        }

        let reg = &self.ep_regs()[ep as usize];

        match reg.read().stat_tx().bits().into() {
            EndpointStatus::Valid => return Err(UsbError::Busy),
            EndpointStatus::Disabled => return Err(UsbError::InvalidEndpoint),
            _ => {},
        };

        let pmem = self.packet_mem.borrow();
        let bd = &pmem.descrs()[ep as usize];

        // TODO: validate len

        pmem.write(bd.addr_tx.get(), buf);
        bd.count_tx.set(buf.len());

        reg.set_stat_tx(EndpointStatus::Valid);

        Ok(0)
    }

    fn read(&self, ep: u8, buf: &mut [u8]) -> Result<usize> {
        if ep & 0x80 != 0 || ep as usize >= NUM_ENDPOINTS {
            return Err(UsbError::InvalidEndpoint);
        }

        let reg = &self.ep_regs()[ep as usize];

        let reg_v = reg.read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled {
            return Err(UsbError::InvalidEndpoint);
        }

        if !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::NoData);
        }

        let pmem = self.packet_mem.borrow();
        let bd = &pmem.descrs()[ep as usize];

        let count = bd.count_rx.get() & 0x3f;
        if count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        pmem.read(bd.addr_rx.get(), buf);

        reg.clear_ctr_rx();
        reg.set_stat_rx(EndpointStatus::Valid);

        return Ok(count)
    }

    fn stall(&self, ep: u8) {
        if ep & 0x80 != 0 {
            self.ep_regs()[(ep & !0x80) as usize].set_stat_tx(EndpointStatus::Stall);
        } else {
            self.ep_regs()[ep as usize].set_stat_rx(EndpointStatus::Stall);
        }
    }

    fn unstall(&self, ep: u8) {
        let reg = &self.ep_regs()[(ep & !0x80) as usize];

        if ep & 0x80 != 0 {
            if reg.read().stat_tx().bits() == EndpointStatus::Stall as u8 {
                reg.set_stat_tx(EndpointStatus::Valid);
            }
        } else {
            if reg.read().stat_rx().bits() == EndpointStatus::Stall as u8 {
                reg.set_stat_rx(EndpointStatus::Valid);
            }
        }
    }
}

pub struct Endpoints<'a> {
    pub ep1: EndpointPair<'a, UsbBus>,
    pub ep2: EndpointPair<'a, UsbBus>,
    pub ep3: EndpointPair<'a, UsbBus>,
    pub ep4: EndpointPair<'a, UsbBus>,
    pub ep5: EndpointPair<'a, UsbBus>,
    pub ep6: EndpointPair<'a, UsbBus>,
    pub ep7: EndpointPair<'a, UsbBus>,
}

impl<'a> Endpoints<'a> {
    pub(crate) fn new(bus: &'a UsbBus) -> Endpoints<'a> {
        Endpoints {
            ep1: EndpointPair::new(bus, 1),
            ep2: EndpointPair::new(bus, 2),
            ep3: EndpointPair::new(bus, 3),
            ep4: EndpointPair::new(bus, 4),
            ep5: EndpointPair::new(bus, 5),
            ep6: EndpointPair::new(bus, 6),
            ep7: EndpointPair::new(bus, 7),
        }
    }
}