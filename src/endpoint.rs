use cortex_m::interrupt;
use usb_device::{Result, UsbError, UsbDirection};
use usb_device::endpoint::{EndpointDescriptor, EndpointAddress};
use crate::target::UsbAccessType;
use crate::endpoint_buffer::{EndpointBuffer, BufferDescriptor};
use crate::endpoint_register::{EndpointRegister, EndpointStatus};


pub struct EndpointOut {
    pub(crate) descriptor: EndpointDescriptor,
    pub(crate) buf: EndpointBuffer,
}

impl EndpointOut {
    fn reg(&self) -> EndpointRegister {
        EndpointRegister::get(self.descriptor.address.number())
    }

    fn buf_descr(&self) -> &'static BufferDescriptor {
        BufferDescriptor::get(self.descriptor.address.number())
    }
}

impl usb_device::endpoint::Endpoint for EndpointOut {
    fn descriptor(&self) -> &EndpointDescriptor { &self.descriptor }

    fn enable(&mut self) {
        self.reg().configure(self.ep_type());
        self.reg().set_stat_rx(EndpointStatus::Valid);
        self.reg().clear_ctr_rx();
    }

    fn disable(&mut self) {
        self.reg().set_stat_rx(EndpointStatus::Disabled);
        self.reg().clear_ctr_rx();
    }

    fn set_stalled(&mut self, stalled: bool) {
        set_stalled(self.address(), stalled);
    }

    fn is_stalled(&self) -> bool {
        is_stalled(self.address())
    }
}

impl usb_device::endpoint::EndpointOut for EndpointOut {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let reg_v = self.reg().read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::WouldBlock);
        }

        self.reg().clear_ctr_rx();

        let count = (self.buf_descr().count_rx.get() & 0x3ff) as usize;
        if count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.buf.read(&mut buf[0..count]);

        self.reg().set_stat_rx(EndpointStatus::Valid);

        Ok(count)
    }
}

pub struct EndpointIn {
    pub(crate) descriptor: EndpointDescriptor,
    pub(crate) buf: EndpointBuffer,
}

impl EndpointIn {
    fn reg(&self) -> EndpointRegister {
        EndpointRegister::get(self.descriptor.address.number())
    }

    fn buf_descr(&self) -> &'static BufferDescriptor {
        BufferDescriptor::get(self.descriptor.address.number())
    }
}

impl usb_device::endpoint::Endpoint for EndpointIn {
    fn descriptor(&self) -> &EndpointDescriptor { &self.descriptor }

    fn enable(&mut self) {
        self.reg().configure(self.ep_type());
        self.reg().set_stat_tx(EndpointStatus::Nak);
        self.reg().clear_ctr_tx();
    }

    fn disable(&mut self) {
        self.reg().set_stat_tx(EndpointStatus::Disabled);
        self.reg().clear_ctr_tx();
    }

    fn set_stalled(&mut self, stalled: bool) {
        set_stalled(self.address(), stalled);
    }

    fn is_stalled(&self) -> bool {
        is_stalled(self.address())
    }
}

impl usb_device::endpoint::EndpointIn for EndpointIn {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        if buf.len() > self.buf.capacity() {
            return Err(UsbError::BufferOverflow);
        }

        match self.reg().read().stat_tx().bits().into() {
            EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
            _ => {},
        };

        self.buf.write(buf);
        self.buf_descr().count_tx.set(buf.len() as u16 as UsbAccessType);

        self.reg().set_stat_tx(EndpointStatus::Valid);

        Ok(())
    }
}

pub(crate) fn set_stalled(ep_addr: EndpointAddress, stalled: bool) {
    interrupt::free(|_| {
        let mut epr = EndpointRegister::get(ep_addr.number());

        match (stalled, ep_addr.direction()) {
            (true, _) => epr.set_stat_tx(EndpointStatus::Stall),
            (false, UsbDirection::In) => epr.set_stat_tx(EndpointStatus::Nak),
            (false, UsbDirection::Out) => epr.set_stat_rx(EndpointStatus::Valid),
        };
    });
}

pub(crate) fn is_stalled(ep_addr: EndpointAddress) -> bool {
    let epr_v = EndpointRegister::get(ep_addr.number()).read();

    let status = match ep_addr.direction() {
        UsbDirection::In => epr_v.stat_tx().bits(),
        UsbDirection::Out => epr_v.stat_rx().bits(),
    };

    status == (EndpointStatus::Stall as u8)
}

pub(crate) fn calculate_count_rx(mut size: usize) -> Result<(usize, u16)> {
    if size <= 62 {
        // Buffer size is in units of 2 bytes, 0 = 0 bytes
        size = (size + 1) & !0x01;

        let size_bits = size >> 1;

        Ok((size, (size_bits << 10) as u16))
    } else if size <= 1024 {
        // Buffer size is in units of 32 bytes, 0 = 32 bytes
        size = (size + 31) & !0x1f;

        let size_bits = (size >> 5) - 1;

        Ok((size, (0x8000 | (size_bits << 10)) as u16))
    } else {
        Err(UsbError::EndpointMemoryOverflow)
    }
}