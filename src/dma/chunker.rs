use crate::buffer::{self, DeviceBuffer};

/// Hardware processing mode
pub enum HwProcMode {
    /// Always processed in full by hardware
    Committed,
    /// Processed best-effort, and HW may partially perform DMA
    BestEffort,
}

pub struct DmaChunker {
    mode: HwProcMode,
    buffer: DeviceBuffer,
    active: usize,
    transferred: usize,
}
impl DmaChunker {
    pub fn new(mode: HwProcMode, buffer: DeviceBuffer) -> Self {
        Self {
            mode,
            buffer,
            active: 0,
            transferred: 0,
        }
    }
}
