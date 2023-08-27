use std::io::{Error, ErrorKind};
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DevicePortStatus {
    DevAvailable,
    DevUsed,
    DevError,
    PortNull,
    PortNotAssigned,
    PortUsed,
    PortError,
}
impl DevicePortStatus {
    pub fn description(&self) -> String {
        match self {
            DevicePortStatus::DevAvailable => "Device Available",
            DevicePortStatus::DevUsed => "Device in Use",
            DevicePortStatus::DevError => "Device Error",
            DevicePortStatus::PortNull => "Port Available",
            DevicePortStatus::PortNotAssigned => "Port Initializing",
            DevicePortStatus::PortUsed => "Port in Use",
            DevicePortStatus::PortError => "Port Error",
        }
        .to_string()
    }
}

impl TryFrom<u64> for DevicePortStatus {
    type Error = std::io::Error;
    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(DevicePortStatus::DevAvailable),
            1 => Ok(DevicePortStatus::DevUsed),
            2 => Ok(DevicePortStatus::DevError),
            3 => Ok(DevicePortStatus::PortNull),
            4 => Ok(DevicePortStatus::PortNotAssigned),
            5 => Ok(DevicePortStatus::PortUsed),
            6 => Ok(DevicePortStatus::PortError),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid Status number.",
            )),
        }
    }
}

impl TryFrom<u32> for DevicePortStatus {
    type Error = std::io::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}
impl TryFrom<u16> for DevicePortStatus {
    type Error = std::io::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}
impl TryFrom<u8> for DevicePortStatus {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}

impl Into<u8> for DevicePortStatus {
    fn into(self) -> u8 {
        match self {
            DevicePortStatus::DevAvailable => 0,
            DevicePortStatus::DevUsed => 1,
            DevicePortStatus::DevError => 2,
            DevicePortStatus::PortNull => 3,
            DevicePortStatus::PortNotAssigned => 4,
            DevicePortStatus::PortUsed => 5,
            DevicePortStatus::PortError => 6,
        }
    }
}

impl Into<u64> for DevicePortStatus {
    fn into(self) -> u64 {
        Into::<u8>::into(self) as u64
    }
}

impl Into<u32> for DevicePortStatus {
    fn into(self) -> u32 {
        Into::<u8>::into(self) as u32
    }
}

impl Into<u16> for DevicePortStatus {
    fn into(self) -> u16 {
        Into::<u8>::into(self) as u16
    }
}
