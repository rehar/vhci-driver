use std::io::{Error, ErrorKind};

use udev::Device;

use crate::{DevicePortStatus, usb_device::{UsbDevice}, UsbSpeed, Port, DevId, BusNum, DevNum};

#[derive(Debug, Clone)]
pub struct ImportedDevice {
    pub hub: UsbSpeed,
    pub port: Port,
    pub status: DevicePortStatus,
    pub devid: DevId,
    pub busnum: BusNum,
    pub devnum: DevNum,
    pub udev: Option<UsbDevice>,
}

impl ImportedDevice {
    pub(crate) fn from_status_str(value: &str) -> std::io::Result<Self> {
        let elements = value
            .to_owned()
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let elements = elements
            .iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<&String>>();

        if elements.len() != 7 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Input does not match VHCI Status format.",
            ));
        }

        let speed_class = UsbSpeed::from_vhci_status(elements[0].clone());

        let port = elements[1].parse::<u8>().or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Unable to parse port.",
        )))?;

        let status: DevicePortStatus = elements[2]
            .parse::<u32>()
            .or(Err(Error::new(
                ErrorKind::InvalidInput,
                "Unable to parse status.",
            )))?
            .try_into()?;

        let _speed = elements[3].parse::<u32>().or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Unable to parse speed.",
        )))?;

        let devid = u64::from_str_radix(&elements[4], 16).or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Unable to parse device id.",
        )))? as u32;

        let _socket = elements[5].parse::<u32>().or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Unable to parse busid.",
        )))?;

        let busid = elements[6].clone();
        let busnum = (devid >> 16) as BusNum;
        let devnum = (devid & 0x0000ffff) as DevNum;

        if status != DevicePortStatus::PortNull && status != DevicePortStatus::PortNotAssigned {
            let vdev = Device::from_subsystem_sysname("usb".into(), busid)
                .map(|dev| Into::<UsbDevice>::into(&dev))
                .ok();

            Ok(Self {
                hub: speed_class,
                port,
                status,
                devid,
                busnum,
                devnum,
                udev: vdev,
            })
        } else {
            Ok(Self {
                hub: speed_class,
                port,
                status,
                devid,
                busnum,
                devnum,
                udev: None,
            })
        }
    }
}
