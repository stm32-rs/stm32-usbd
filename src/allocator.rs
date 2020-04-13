use crate::endpoint::{EndpointIndex, NUM_ENDPOINTS, UsbEndpointOut, UsbEndpointIn};
use crate::usbcore::UsbCore;
use crate::UsbPeripheral;
use usb_device::{Result, UsbDirection, UsbError};
use usb_device::endpoint::{EndpointConfig, EndpointType};
use usb_device::usbcore;

// This allocator has to implement some quirky logic because the transfer type field is shared
// between IN and OUT of the same endpoint index. Currently it will never allocate endpoints from
// two different interfaces to the same register pair. Two endpoints from different alternate
// settings of the same interface may be allocated to the same endpoint, because they will never be
// enabled at the same time.

#[derive(Default)]
pub(crate)struct EpPairConfig {
    pub iface: Option<u8>,
    pub type_in_alt: Option<EndpointType>,
}

#[derive(Default)]
pub struct UsbEndpointAllocator {
    iface: u8,
    //ep_type_in_alt: [Option<EndpointType>; NUM_ENDPOINTS],
    pub(crate) eps: [EpPairConfig; NUM_ENDPOINTS],
    pub(crate) max_packet_size_out: [u16; NUM_ENDPOINTS],
    pub(crate) max_packet_size_in: [u16; NUM_ENDPOINTS],
    /*pub(crate) ep_out: [EpConfig; NUM_ENDPOINTS],
    pub(crate) ep_in: [EpConfig; NUM_ENDPOINTS],*/
}

impl UsbEndpointAllocator {
    fn alloc(&mut self, dir: UsbDirection, config: &EndpointConfig) -> Result<(EndpointIndex, u16)> {
        let max_packet_size = if dir == UsbDirection::Out {
            &mut self.max_packet_size_out
        } else {
            &mut self.max_packet_size_in
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

            let ep = &mut self.eps[i];

            if ep.iface.map(|i| i != iface).unwrap_or(false)
                || ep.type_in_alt.map(|t| t != config.ep_type()).unwrap_or(false)
            {
                continue;
            }

            ep.iface = Some(self.iface);
            ep.type_in_alt = Some(config.ep_type());

            // Round to the nearest even size
            let size = (config.max_packet_size() + 1) & !1;

            if size > max_packet_size[i] {
                max_packet_size[i] = size;
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
            self.eps[i].type_in_alt = None;
        }

        Ok(())
    }
}
