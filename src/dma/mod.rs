//!
//! Direct memory access (DMA)
//!
//! DMA is how we interact with the actual TPU chip.
//!
//! If implemented properly, it's the best way to do it.
//!

mod chunker;

use std::ops::{Add, BitAnd, Deref, Div, Mul, Shl, Shr, Sub};

use crate::{buffer::DeviceBuffer, error::Error};

/// A virtual address
#[derive(Clone, Copy)]
pub struct Address(u64);
impl From<u64> for Address {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl Into<u64> for Address {
    fn into(self) -> u64 {
        self.0
    }
}
impl Deref for Address {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Add<u64> for Address {
    type Output = Address;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl Sub<u64> for Address {
    type Output = Address;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}
impl Mul<u64> for Address {
    type Output = Address;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Div<u64> for Address {
    type Output = Address;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl Shl<u64> for Address {
    type Output = u64;

    fn shl(self, rhs: u64) -> Self::Output {
        self.0 << rhs
    }
}
impl Shr<u64> for Address {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 << rhs
    }
}
impl BitAnd<u64> for Address {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

#[derive(Debug)]
pub enum DescriptorKind {
    Instruction,
    InputActivation,
    Parameter,
    OutputActivation,
    ScalarCoreInterrupt0,
    ScalarCoreInterrupt1,
    ScalarCoreInterrupt2,
    ScalarCoreInterrupt3,
    /// Used internally in the DMA scheduler to sync DMAs local to a Request
    LocalFence,
    /// Used internally in the DMA scheduler to sync DMAs across Requests
    GlobalFence,
}

pub enum DmaState {
    Pending,
    Active,
    Completed,
    Error,
}

#[repr(C)]
pub enum DmaDirection {
    /// Caches flushed at mapping time
    ToDev = 1,
    /// Caches invalidated at unmapping time
    FromDev = 2,
    /// Caches flushed at mapping time and invalidated at unmapping time
    Bi = 0,
}

//pub struct Dma {
//    id: i32,
//    kind: DescriptorKind,
//    state: DmaState,
//    buf: DeviceBuffer,
//}

pub struct DmaInfo {
    id: u32,
    kind: DescriptorKind,
    state: DmaState,
    buf: DeviceBuffer,
}
impl DmaInfo {
    pub fn new(id: u32, kind: DescriptorKind, buf: DeviceBuffer) -> Self {
        Self {
            id,
            kind,
            state: DmaState::Pending,
            buf,
        }
    }
}

pub trait Dma {
    fn read_32(&self, addr: Address) -> Result<u32, Error>;
    fn read(&self, addr: Address) -> Result<u64, Error>;
    fn write_32(&self, addr: Address, val: u32) -> Result<(), Error>;
    fn write(&self, addr: Address, val: u64) -> Result<(), Error>;
}
