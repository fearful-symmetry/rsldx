use rusb::{UsbContext, Device, DeviceDescriptor, TransferType, Direction, GlobalContext, devices};
use usb_ids::FromId;
use log::debug;

use crate::errors::*;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    pub config: u8,
    pub interface: u8,
    pub setting: u8,
    pub address: u8,
}

pub fn find_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    transfer_type: TransferType,
    transfer_direction: Direction
) -> Result<Option<UsbEndpoint>, DisplayError> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = device.config_descriptor(n).context(DeviceFindSnafu)?;

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == transfer_direction
                        && endpoint_desc.transfer_type() == transfer_type
                    {
                        return Ok(Some(UsbEndpoint {
                            config: config_desc.number(),
                            interface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                        }));
                    }
                }
            }
        }
    }

    Ok(None)
}

pub fn find_first_device() ->Result<Device<GlobalContext>, DisplayError> {
    let vend_id = 0x0fa8;
    let prod_id = 0xa090;
    let dev_list = devices().context(DeviceListSnafu)?;
    for device in dev_list.iter() {
        let dev_desc = device.device_descriptor().context(DescriptorSnafu)?;
        if dev_desc.vendor_id() == vend_id && dev_desc.product_id() == prod_id {
            let dbg_str = get_device_string(&device)?;
            debug!("Using device {}", dbg_str);
            return Ok(device);
        }
    };
    Err(DisplayError::NoDevicesFound)
}

pub fn get_device_string<T: UsbContext>(device: &Device<T>) -> Result<String, DisplayError> {
    let device_desc = device.device_descriptor().context(DescriptorSnafu)?;

    let vendor_name = match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
        Some(vendor) => vendor.name(),
        None => "Unknown vendor",
    };

    let product_name =
    match usb_ids::Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
        Some(product) => product.name(),
        None => "",
    };
    
    Ok(format!("Bus {:03} Device {:03} ID {:04x}:{:04x} ({} {})",
        device.bus_number(),
        device.address(),
        device_desc.vendor_id(),
        device_desc.product_id(), vendor_name, product_name ))
}