use std::fs::{self};
use std::io::Error;
use std::io::{ErrorKind, Result};
use std::net::TcpStream;
use std::os::fd::{AsFd, AsRawFd};
use udev::Device;
use crate::usb_device::UsbDevice;
use crate::{UsbSpeed, DevicePortStatus, ImportedDevice, Port, BusNum, DevNum, DevId};
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
    
    pub fn get_free_port(&self, speed: UsbSpeed) -> Result<Port>{

        let devices = self.imported_device_list()?;
        let hub_speed = match speed {
            UsbSpeed::Super => UsbSpeed::Super,
            _       => UsbSpeed::High,
        };

        for dev in devices {
            if dev.status == DevicePortStatus::PortNotAssigned && dev.hub == hub_speed {
                return Ok(dev.port);
            }
        }

        Err(Error::new(ErrorKind::NotFound, "No free port available."))
    }

    pub fn attach_device_to_port(&mut self, stream: &TcpStream, busnum: BusNum, devnum: DevNum, speed: UsbSpeed,  port: Port) -> Result<()>{
        let socket = stream.as_fd().as_raw_fd();
        let devid = ((busnum as DevId) << 16) | (devnum as DevId);

        let str = format!("{} {} {} {}", port, socket, devid, Into::<u8>::into(speed));
        self.udev.set_attribute_value("attach", str.to_string())
    }

    pub fn attach_to_port(&mut self, stream: &TcpStream, device: UsbDevice, port: Port) -> Result<()> {      
        self.attach_device_to_port(stream, device.busnum, device.devnum, device.speed, port)
    }

    /// Attaches a USB device to the local Virtual Host Controller Interface
    /// using an existing socket connection `stream` and the USB `device` specification.
    /// 
    /// Returns the port it attached the device to
    pub fn attach(&mut self, stream: &TcpStream, device: UsbDevice) -> Result<Port> {
            let port  = self.get_free_port(device.speed)?;
            self.attach_to_port(stream, device, port)?;
            Ok(port)
    }
    
    pub fn detach(&mut self, port: Port) -> Result<()> {
        self.udev.set_attribute_value("detach", port.to_string())
    }

    // todo make macro
    pub(crate) fn read_attr_default(udev: &Device, attr: &str, default: u64) -> u64 {
        udev.attribute_value(attr)
            .and_then(|s| s.to_str())
            .and_then(|s| u64::from_str_radix(s, 16).ok())
            .unwrap_or(default)
    }
}


