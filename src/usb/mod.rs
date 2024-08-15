//!
//! # Coral USB Accelerator Driver
//!
//! Like the official driver, this uses libusb and doesn't require the kernel module.
//!

use rusb::{Device, DeviceHandle, GlobalContext};

use crate::error::Error;

mod dfu;
mod io;
mod ml;
mod standard;

pub struct EdgeTpuUsb {
    dev: DeviceHandle<GlobalContext>,
}
impl EdgeTpuUsb {
    pub fn new(dev: Device<GlobalContext>) -> Result<Self, Error> {
        let dev = dev.open()?;

        // ML interface
        dev.claim_interface(0)?;

        //top level handler

        Ok(Self { dev })
    }
}
