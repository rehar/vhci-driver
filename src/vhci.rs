use std::fs::{self};
use std::io::Error;
use std::io::{ErrorKind, Result};
use std::net::TcpStream;
use std::os::fd::{AsFd, AsRawFd};
use udev::Device;
use crate::udev_extension::UdevExtension;
use crate::{UsbSpeed, DevicePortStatus, UsbDevice, ImportedDevice};
pub const USBIP_VHCI_BUS_TYPE: &str = "platform";
pub const USBIP_VHCI_DEVICE_NAME: &str = "vhci_hcd.0";
pub const USBIP_VHCI_DEVICE_NAME_PREFIX: &str = "vhci_hcd.";

#[derive(Debug, Clone)]
pub struct Vhci {
    udev: Device,
    num_ports: usize,
    num_ctrls: usize,
}

impl Vhci {
    pub fn new() -> Result<Self> {
        let dev = Device::from_subsystem_sysname(
                USBIP_VHCI_BUS_TYPE.into(), 
                USBIP_VHCI_DEVICE_NAME.into())
            .map_err(|_| 
                Error::new(
                    ErrorKind::Other, 
                    "Unable to communicate with VHCI kernel driver. Make sure the module is loaded and you have proper permission."
                )
            )?;
        let mut vdev = Self {
            udev: dev,
            num_ports: 0,
            num_ctrls: 0,
        };
        vdev.num_ports = vdev
            .udev_nports()
            .ok_or(Error::new(ErrorKind::InvalidInput, "No available Ports."))?;
        vdev.num_ctrls = vdev.udev_nctrls()?;

        // make sure we can read all imported devices
        let _ = vdev.imported_device_list()?;

        Ok(vdev)
    }

    fn udev_nports(&self) -> Option<usize> {
        self.udev
            .attribute_value("nports")
            .and_then(|s| s.to_str())
            .and_then(|s| { s.parse::<usize>() }.ok())
    }

    fn udev_nctrls(&self) -> Result<usize> {
        let parent = self.udev.parent().ok_or(Error::new(ErrorKind::Other, ""))?;
        let paths = fs::read_dir(parent.syspath())?;
        let entries: Vec<String> = paths
            .filter_map(|p| {
                p.as_ref().map_or(None, |dir_entry| {
                    dir_entry.file_name().to_str().map_or(None, |name| {
                        name.to_string()
                            .contains(USBIP_VHCI_DEVICE_NAME_PREFIX)
                            .then(|| name.to_string())
                    })
                })
            })
            .collect();
        Ok(entries.len())
    }
    fn imported_device_list(&self) -> Result<Vec<ImportedDevice>> {
        let lines = self
            .udev
            .attribute_value("status")
            .and_then(|s| s.to_str())
            .ok_or(Error::new(ErrorKind::Other, "Unable to read VHCI status."))?
            .to_string()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let devices = lines
            .iter()
            .filter_map(|s| ImportedDevice::from_status_str(s).ok())
            .collect::<Vec<ImportedDevice>>();

        Ok(devices)
    }

    pub fn attached_devices(&self) -> Result<Vec<ImportedDevice>> {
        let devices = self.imported_device_list()?;
        Ok(devices.into_iter().filter(|x|x.udev.is_some() ).collect())
    }
    
    pub fn get_free_port(&self, speed: UsbSpeed) -> Result<u8>{

        let devices = self.imported_device_list()?;
        let hub_speed = match speed {
            UsbSpeed::Super => UsbSpeed::Super,
            _       => UsbSpeed::High,
        };

        for dev in devices {
            if dev.status == DevicePortStatus::PortNull && dev.hub == hub_speed {
                return Ok(dev.port);
            }
        }

        Err(Error::new(ErrorKind::NotFound, "No free port available."))
    }

    pub fn attach_device_to_port(&mut self, stream: &TcpStream, busnum: u32, devnum: u32, speed: UsbSpeed,  port: u8) -> Result<()>{
        let socket = stream.as_fd().as_raw_fd();
        let devid = (busnum << 16) | devnum;

        let str = format!("{} {} {} {}", port, socket, devid, Into::<u8>::into(speed));
        self.udev.set_attribute_value("attach", str.to_string())
    }

    pub fn attach_to_port(&mut self, stream: &TcpStream, device: UsbDevice, port: u8) -> Result<()> {      
        self.attach_device_to_port(stream, device.busnum, device.devnum, device.speed, port)
    }

    pub fn attach(&mut self, stream: &TcpStream, device: UsbDevice) -> Result<()> {
            let port  = self.get_free_port(device.speed)?;
            self.attach_to_port(stream, device, port)
    }
    
    pub fn detach(&mut self, port: u8) -> Result<()> {
        self.udev.set_attribute_value("detach", port.to_string())
    }

    // todo make macro
    fn read_attr_default(udev: &Device, attr: &str, default: u64) -> u64 {
        udev.attribute_value(attr)
            .and_then(|s| s.to_str())
            .and_then(|s| u64::from_str_radix(s, 16).ok())
            .unwrap_or(default)
    }
}

impl From<udev::Device> for UsbDevice {
    fn from(value: udev::Device) -> Self {
        let busid = value.sysname().to_str().unwrap_or("").to_string();

        // parse busnum and devnum from busid
        let ids: Vec<u32> = busid
            .split("-")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let (busnum, devnum) = if ids.len() == 2 {
            (ids[0], ids[1])
        } else {
            // default
            (0, 0)
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
            busnum,
            devnum,
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
