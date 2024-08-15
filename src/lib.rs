//!
//! # Edged
//!
//! Edged is an unofficial driver for Coral Edge TPU devices, written in Rust.
//!
//! `libedgetpu` was "reverse engineered" and used for reference, as no other documentation on Coral
//! devices exists.
//!
//! Edged and `libedgetpu` may have some behavioral differences, but shouldn't do anything that may
//! risk bricking.
//! The same goes for harming the device, except in unrestricted mode.
//!
//! ## Supported devices
//! Only EdgeTPUv1 will be supported, unless Google revives Coral.
//!  - [ ] M.2
//!  - [ ] M.2 Dual
//!  - [ ] USB
//!  - [-] Dev boards
//!  - [-] Standalone
//!
//! Internal TPU projects at Google will never be supported in any way, because I don't want to aid
//! them in abusing Internet users and making the web a worse place.
//!
//! ## License
//! This library is licensed AGPL, because I don't want Google using my crap to abuse Internet
//! users.
//!

//use dma::Dma;
use error::Error;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rusb;

// A safety measure probably worth including
#[cfg(all(
    feature = "unrestricted",
    not(feature = "I_UNDERSTAND_WHAT_I_AM_DOING_DADDY_GOOGLE")
))]
compile_error!(
    r#"
If you're using the Coral USB Accelerator, it may heat up during operation,
depending on the computation workloads and operating frequency. Touching the
metal part of the USB Accelerator after it has been operating for an extended
period of time may lead to discomfort and/or skin burns. As such, if you enable
the Edge TPU runtime using the maximum operating frequency, the USB Accelerator
should be operated at an ambient temperature of 25°C or less. Alternatively, if
you enable the Edge TPU runtime using the reduced operating frequency, then the
device is intended to safely operate at an ambient temperature of 35°C or less.

If you've read and understand this, enable the 'I_UNDERSTAND_WHAT_I_AM_DOING_DADDY_GOOGLE' feature.
"#
);

//pub(crate) mod buffer;
//pub(crate) mod chip;
//pub(crate) mod dma;
pub(crate) mod error;
pub mod flatbuffers;
//mod interrupt;
//pub(crate) mod pcie;
//mod request;
//pub(crate) mod usb;
//mod executable;
//mod package;
//mod memory;

//pub(crate) trait Register {}
//impl Register for u32 {}
//impl Register for u64 {}
//
//pub struct CoralDevice {}
//
//pub fn run_control<D: Dma>(dma: D) -> Result<(), Error> {
//    use chip::offsets::*;
//
//    const INVALID_OFFSET: u64 = -1i64 as u64;
//
//    let val = 0u64;
//
//    for offset in [
//        SCALARCORE_RUN_CONTROL,
//        AVDATAPOP_RUN_CONTROL,
//        PARAMETERPOP_RUN_CONTROL,
//        INFEED_RUN_CONTROL,
//        OUTFEED_RUN_CONTROL,
//    ] {
//        dma.write(offset, val)?;
//    }
//
//    // tile config junk
//    dma.write(TILECONFIG0, val /* helper.raw() */);
//
//    for offset in [
//        OP_RUN_CONTROL,
//        NARROW_TO_WIDE_RUN_CONTROL,
//        WIDE_TO_NARROW_RUN_CONTROL,
//    ] {
//        dma.write(offset, val)?;
//    }
//
//    // tile thread crap maybe
//
//    for offset in [
//        MESHBUS0_RUN_CONTROL,
//        MESHBUS1_RUN_CONTROL,
//        MESHBUS2_RUN_CONTROL,
//        MESHBUS3_RUN_CONTROL,
//        RINGBUSCONSUMER0_RUN_CONTROL,
//        RINGBUSCONSUMER1_RUN_CONTROL,
//    ] {
//        dma.write(offset, val)?;
//    }
//
//    Ok(())
//}
