use libudev_sys as ffi;
use std::ffi::CString;
use udev::FromRawWithContext;
pub(crate) trait UdevExtension {
    fn from_subsystem_sysname(subsystem: String, sysname: String) -> std::io::Result<udev::Device>;
}

impl UdevExtension for udev::Device {
    fn from_subsystem_sysname(subsystem: String, sysname: String) -> std::io::Result<udev::Device> {
        let subsystem = CString::new(subsystem.as_bytes())
            .ok()
            .ok_or(std::io::Error::from_raw_os_error(libc::EINVAL))?;

        let sysname = CString::new(sysname.as_bytes())
            .ok()
            .ok_or(std::io::Error::from_raw_os_error(libc::EINVAL))?;

        // create udev context
        let udev_ctx = unsafe { ffi::udev_new() }; // udev context;

        if udev_ctx.is_null() {
            return Err(std::io::Error::last_os_error());
        }

        let udev_ptr = unsafe {
            ffi::udev_device_new_from_subsystem_sysname(
                udev_ctx,
                subsystem.as_ptr(),
                sysname.as_ptr(),
            )
        };

        if udev_ptr.is_null() {
            return Err(std::io::Error::last_os_error());
        }

        Ok(unsafe { udev::Device::from_raw_with_context(udev_ctx, udev_ptr) })
    }
}
