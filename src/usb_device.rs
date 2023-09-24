use std::path::PathBuf;

use crate::{UsbSpeed, DevNum, BusNum, Vhci};

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub path: PathBuf,
    pub busid: String,
    pub busnum: BusNum,
    pub devnum: DevNum,
    pub speed: UsbSpeed,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub b_device_class: u8,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_configuration_value: u8,
    pub b_num_configurations: u8,
    pub b_num_interfaces: u8,
}
impl UsbDevice {
    pub fn from_udev(udev: udev::Device) -> Self {
        udev.into()
    }
}

impl From<udev::Device> for UsbDevice {
    fn from(value: udev::Device) -> Self {
        (&value).into()
    }
}

impl From<&udev::Device> for UsbDevice {
    fn from(value: &udev::Device) -> Self {

        let busid = value.sysname().to_str().unwrap_or("").to_string();

        // parse busnum and devnum from busid
        let ids: Vec<u32> = busid
            .split("-")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let busnum = if ids.len() >= 1 {
            ids[0]
        } else {
            // default
            0
        };

        let speed = value
            .attribute_value("speed")
            .and_then(|s| s.to_str())
            .map_or(UsbSpeed::Unknown, |s| {
                UsbSpeed::from_udev_speed(s.to_string())
            });

        Self {
            path: value.syspath().to_path_buf(),
            busid: busid,
            busnum: busnum as BusNum,
            devnum: Vhci::read_attr_default(&value, "devnum", 0) as DevNum,
            speed: speed,
            id_vendor: Vhci::read_attr_default(&value, "idVendor", 0) as u16,
            id_product: Vhci::read_attr_default(&value, "idProduct", 0) as u16,
            bcd_device: Vhci::read_attr_default(&value, "bcdDevice", 0) as u16,
            b_device_class: Vhci::read_attr_default(&value, "bDeviceClass", 0) as u8,
            b_device_sub_class: Vhci::read_attr_default(&value, "bDeviceSubClass", 0) as u8,
            b_device_protocol: Vhci::read_attr_default(&value, "bDeviceProtocol", 0) as u8,
            b_configuration_value: Vhci::read_attr_default(&value, "bConfigurationValue", 0) as u8,
            b_num_configurations: Vhci::read_attr_default(&value, "bNumConfigurations", 0) as u8,
            b_num_interfaces: Vhci::read_attr_default(&value, "bNumInterfaces", 0) as u8,
        }
    }
}
