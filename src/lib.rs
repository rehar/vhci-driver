mod vhci;
mod usb_speed;
mod usb_device;
mod device_port_status;
mod imported_device;

pub type Port = u8;
pub type DevId = u32;
pub type BusNum = u32;
pub type DevNum = u32;

pub use vhci::*;
pub use usb_speed::*;
pub use usb_device::*;
pub use device_port_status::*;
pub use imported_device::*;