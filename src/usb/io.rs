use crate::dma::{self, Dma};

pub enum Kind {
    /// Host -> Device
    BulkOut,
    /// Host <- Device
    BulkIn,
    /// Host <- Device
    ScHostInterrupt,
}
impl From<dma::DescriptorKind> for Kind {
    fn from(value: dma::DescriptorKind) -> Self {
        use dma::DescriptorKind::*;

        match value {
            Instruction | InputActivation | Parameter => Self::BulkOut,
            OutputActivation => Self::BulkIn,
            ScalarCoreInterrupt0 | ScalarCoreInterrupt1 | ScalarCoreInterrupt2
            | ScalarCoreInterrupt3 => Self::ScHostInterrupt,
            _ => panic!("can't be converted: {value:?}"),
        }
    }
}

pub struct IoRequest {
    id: i32,
    kind: Kind,
    //dma chunker
    //dma info
    dma: Dma,
    //header
}
