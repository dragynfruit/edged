use std::time::Duration;

use rusb::{request_type, Direction, Recipient, RequestType};

use crate::{
    dma::{Address, Dma},
    error::Error,
};

use super::EdgeTpuUsb;

#[repr(C)]
enum DescriptorTag {
    Unknown = -1,
    Instructions = 0,
    InputActivations = 1,
    Parameters = 2,
    OutputActivations = 3,
    Interrupt0 = 4,
    Interrupt1 = 5,
    Interrupt2 = 6,
    Interrupt3 = 7,
}

const SINGLE_BULK_OUT_ENDPOINT: u8 = 1;

impl EdgeTpuUsb {
    /// Write a header packet
    pub(crate) fn write_header(&self, tag: DescriptorTag, len: u32) -> Result<(), Error> {
        let mut buf = [0u8; 8];

        buf[0..4].copy_from_slice(&len.to_le_bytes());
        buf[4] = (tag as u8) & 0xF;
        // [5..] seems unused

        self.dev.write_bulk(SINGLE_BULK_OUT_ENDPOINT, &buf, Duration::from_secs(6))?;

        Ok(())
    }
}
impl Dma for EdgeTpuUsb {
    /// Read a 32-bit value from a register at the given offset
    fn read_32(&self, addr: Address) -> Result<u32, Error> {
        let buf = &mut [0u8; 4];

        self.dev.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Device),
            1,
            (addr & 0xFFFF) as u16,
            (addr >> 16) as u16,
            buf,
            Duration::from_secs(6),
        )?;

        Ok(u32::from_le_bytes(*buf))
    }

    /// Read a 64-bit value from a register at the given offset
    fn read(&self, addr: Address) -> Result<u64, Error> {
        let buf = &mut [0u8; 8];

        self.dev.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Device),
            0,
            (addr & 0xFFFF) as u16,
            (addr >> 16) as u16,
            buf,
            Duration::from_secs(6),
        )?;

        Ok(u64::from_le_bytes(*buf))
    }

    /// Write a 32-bit value to a register at the given offset
    fn write_32(&self, addr: Address, val: u32) -> Result<(), Error> {
        self.dev.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
            1,
            (addr & 0xFFFF) as u16,
            (addr >> 16) as u16,
            &val.to_le_bytes(),
            Duration::from_secs(6),
        )?;

        Ok(())
    }

    /// Write a 64-bit value to a register at the given offset
    fn write(&self, addr: Address, val: u64) -> Result<(), Error> {
        self.dev.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
            0,
            (addr & 0xFFFF) as u16,
            (addr >> 16) as u16,
            &val.to_le_bytes(),
            Duration::from_secs(6),
        )?;

        Ok(())
    }
}
