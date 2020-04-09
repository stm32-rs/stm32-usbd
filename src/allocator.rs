use crate::endpoint::{EndpointIndex, NUM_ENDPOINTS, UsbEndpointOut, UsbEndpointIn};
use crate::usbcore::UsbCore;
use crate::UsbPeripheral;
use usb_device::{Result, UsbDirection, UsbError};
use usb_device::endpoint::{EndpointConfig, EndpointType};
use usb_device::usbcore;

#[derive(Default)]
pub(crate) struct EpConfig {
    pub iface: Option<u8>,
    pub max_packet_size: u16,
}

#[derive(Default)]
pub struct UsbEndpointAllocator {
    iface: u8,
    ep_type_in_alt: [Option<EndpointType>; NUM_ENDPOINTS],
    pub(crate) ep_out: [EpConfig; NUM_ENDPOINTS],
    pub(crate) ep_in: [EpConfig; NUM_ENDPOINTS],
}

impl UsbEndpointAllocator {
    fn alloc(&mut self, dir: UsbDirection, config: &EndpointConfig) -> Result<(EndpointIndex, u16)> {
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

            return Ok((unsafe { EndpointIndex::new_unchecked(i as u8) }, size));
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
        usbcore::UsbEndpointAllocator::<UsbCore<USB>>::next_alt_setting(self)
    }

    fn next_alt_setting(&mut self) -> Result<()> {
        for i in 0..NUM_ENDPOINTS {
            self.ep_type_in_alt[i] = None;
        }

        Ok(())
    }
}
