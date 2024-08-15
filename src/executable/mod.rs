mod util;

pub use util::*;

use crate::{buffer::DeviceBuffer, dma::{DescriptorKind, DmaInfo}, flatbuffers::executable::platforms::darwinn::{Description, Executable}};

pub struct DmaInfoExtractor {
}
impl DmaInfoExtractor {
    pub fn extract_instruction_dma_infos(exec: Executable) -> Vec<DmaInfo> {
        let mut dmas = Vec::new();

        let mut id = 0u32;
        for buf in exec.instruction_bitstreams().unwrap() {
            let buf = DeviceBuffer::new(device_address, size)
           dmas.push(DmaInfo::new(id, DescriptorKind::Instruction, buf.));
        }
        // For debugging purposes apparently
        #[cfg(debug_assertions)]
        {
            //dmas.push(id, DescriptorKind::GlobalFence);
        }

        dmas
    }
}
