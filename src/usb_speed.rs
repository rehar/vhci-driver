use std::io::{Error, ErrorKind};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UsbSpeed {
    Unknown,
    Low,
    Full,
    High,
    Wireless,
    Super,
    SuperPlus,
}

impl UsbSpeed {
    pub(crate) fn from_vhci_status(val: String) -> Self {
        match val.to_uppercase().as_str() {
            "HS" => UsbSpeed::High,
            "SS" => UsbSpeed::Super,
            _ => UsbSpeed::Unknown,
        }
    }

    pub fn from_udev_speed(val: String) -> Self {
        match val.trim() {
            "1.5" => UsbSpeed::Low,
            "12" => UsbSpeed::Full,
            "480" => UsbSpeed::High,
            "53.3-480" => UsbSpeed::Wireless,
            "5000" => UsbSpeed::Super,
            "10000" => UsbSpeed::SuperPlus,
            _ => UsbSpeed::Unknown,
        }
    }

    pub fn speed(&self) -> String {
        match self {
            UsbSpeed::Unknown => String::from("unknown"),
            UsbSpeed::Low => String::from("1.5"),
            UsbSpeed::Full => String::from("12"),
            UsbSpeed::High => String::from("480"),
            UsbSpeed::Wireless => String::from("53.3-480"),
            UsbSpeed::Super => String::from("5000"),
            UsbSpeed::SuperPlus => String::from("10000"),
        }
    }

    pub fn description(&self) -> String {
        match self {
            UsbSpeed::Unknown => String::from("Unknown Speed"),
            UsbSpeed::Low => String::from("Low Speed(1.5Mbps)"),
            UsbSpeed::Full => String::from("Full Speed(12Mbps)"),
            UsbSpeed::High => String::from("High Speed(480Mbps)"),
            UsbSpeed::Wireless => String::from("Wireless"),
            UsbSpeed::Super => String::from("Super Speed(5000Mbps)"),
            UsbSpeed::SuperPlus => String::from("Super Speed(10000Mbps)"),
        }
    }
}

impl TryFrom<u64> for UsbSpeed {
    type Error = std::io::Error;
    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Low),
            2 => Ok(Self::Full),
            3 => Ok(Self::High),
            4 => Ok(Self::Wireless),
            5 => Ok(Self::Super),
            6 => Ok(Self::SuperPlus),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid USB Speed.")),
        }
    }
}

impl TryFrom<usize> for UsbSpeed {
    type Error = std::io::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}

impl TryFrom<u32> for UsbSpeed {
    type Error = std::io::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}

impl TryFrom<u16> for UsbSpeed {
    type Error = std::io::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}

impl TryFrom<u8> for UsbSpeed {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok((value as u64).try_into()?)
    }
}

impl Into<usize> for UsbSpeed {
    fn into(self) -> usize {
        Into::<u8>::into(self) as usize
    }
}
impl Into<u64> for UsbSpeed {
    fn into(self) -> u64 {
        Into::<u8>::into(self) as u64
    }
}

impl Into<u32> for UsbSpeed {
    fn into(self) -> u32 {
        Into::<u8>::into(self) as u32
    }
}

impl Into<u16> for UsbSpeed {
    fn into(self) -> u16 {
        Into::<u8>::into(self) as u16
    }
}

impl Into<u8> for UsbSpeed {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::Low => 1,
            Self::Full => 2,
            Self::High => 3,
            Self::Wireless => 4,
            Self::Super => 5,
            Self::SuperPlus => 6,
        }
    }
}
