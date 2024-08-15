//!
//! Gasket kernel driver support
//!
//! The Gasket kernel driver is used to interface with PCIe devices.
//!

use std::{fs::File, os::fd::{AsFd, AsRawFd, IntoRawFd, OwnedFd}, path::{Path, PathBuf}};
use nix::sys::eventfd::EventFd;

use crate::{dma::Address, error::Error};

mod ioctl;
use ioctl::*;

pub struct EdgeTpuPcie {
    fd: OwnedFd,
    evfd: EventFd,
    map_flags_supported: bool,
}
impl EdgeTpuPcie {
    pub fn open(device_path: impl Into<PathBuf>, num_simple_page_table_entries: u64) -> Result<Self, Error> {
        let fd = File::open(device_path.into())?.as_fd().try_clone_to_owned()?;

        let ioctl_buffer = gasket_page_table {
            pt_index: 0,
            size: num_simple_page_table_entries,
            dev_addr: 0,
            host_addr: 0,
        };
        unsafe {
            gasket_partition_page_table(fd.as_raw_fd(), &ioctl_buffer)?;
        }

        let evfd = EventFd::new()?;
        
        Ok(Self {
            fd,
            evfd,
            map_flags_supported: false,
        })
    }
    //pub fn map(&self, num_pages: i32, virtual_address: Address) -> Result<(), Error> {
    pub fn map_buf(&self, num_pages: i32, virtual_address: Address) -> Result<(), Error> {
        unsafe {
            //let buffer_to_map: *const gasket_page_table_ioctl;
            //(*buffer_to_map).page_table_index = 0;
            //(*buffer_to_map).host
            //(*bu
            let buffer_to_map = gasket_page_table {
                pt_index: 0,
                host_addr: buffer.addr(),
                size: num_pages as u64,// * HostPageSize,
                dev_addr: virtual_address.into(),
            };

            gasket_map_buffer(self.fd.as_raw_fd(), &buffer_to_map)?;

            Ok(())
        }
    }
    pub fn unmap_buf(&self, num_pages: i32, virtual_address: Address) -> Result<(), Error> {
        unsafe {
            let buffer_to_map = gasket_page_table {
                pt_index: 0,
                host_addr: buffer.addr(),
                size: num_pages as u64, //* HostPageSize,
                dev_addr: virtual_address,
            };

            gasket_unmap_buffer(self.fd.as_raw_fd(), &buffer_to_map)?;

            Ok(())
        }
    }
}
