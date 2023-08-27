use std::path::PathBuf;

use crate::UsbSpeed;

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub path: PathBuf,
    pub busid: String,
    pub busnum: u32,
    pub devnum: u32,
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