//!
//! Chip config
//!
//! Only EdgeTPUv1 is supported.
//!

use bitflags::bitflags;

/// Minimum alignment for buffers
const MIN_ALIGN_BYTES: u16 = 8;

/// Allocation alignment for buffers
///
/// Usually same as [Chip::MIN_ALIGN_BYTES]
const ALLOC_ALIGN_BYTES: u16 = 4096;

// Number of page table entries.
const PAGE_TABLE_ENTRIES: u16 = 8192;

/// Total size of narrow memory per tile in bytes
const NARROW_MEMORY_CAPACITY: u16 = 196_608;

/// Size of address translation entry for external narrow memory interface
const EXTERNAL_NARROW_MEMORY_TRANSLATE_ENTRY_SIZE: u16 = 262_144;

/// Number of X tiles
const X_TILES: u8 = 4;

/// Number of Y tiles
const Y_TILES: u8 = 4;

/// Number of virtual subscriptions
const RING_VCS: u8 = 8;

/// Base offset for CSR
const CSR_REGION_BASE_OFFSET: u32 = 6_291_456;

/// Size of CSR region
const CSR_REGION_SIZE_BYTES: u32 = 2_097_152;

/// Number of cells in a tile
const NUM_CELLS: u8 = 64;

/// Tile memory's base-and-bound unit size in bytes
const NARROW_MEMORY_PARTITION_UNIT_SIZE: u16 = 65_536;

pub mod offsets {
    use crate::dma::Address;

    // AI copied these for me
    pub const FATAL_ERR_INT_CONTROL: Address = 0x486c0;
    pub const FATAL_ERR_INT_STATUS: Address = 0x486c8;
    pub const SC_HOST_INT_CONTROL: Address = 0x486a0;
    pub const SC_HOST_INT_STATUS: Address = 0x486a8;
    pub const TOP_LEVEL_INT_CONTROL: Address = 0x486b0;
    pub const TOP_LEVEL_INT_STATUS: Address = 0x486b8;
    pub const AVDATAPOP_RUN_CONTROL: Address = 0x44158;
    pub const AVDATAPOP_RUN_STATUS: Address = 0x44168;
    pub const AVDATAPOP_BREAK_POINT: Address = 0x44160;
    pub const INFEED_RUN_CONTROL: Address = 0x441d8;
    pub const INFEED_RUN_STATUS: Address = 0x441e0;
    pub const INFEED_BREAK_POINT: Address = 0x441e8;
    pub const OUTFEED_RUN_CONTROL: Address = 0x44218;
    pub const OUTFEED_RUN_STATUS: Address = 0x44220;
    pub const OUTFEED_BREAK_POINT: Address = 0x44228;
    pub const PARAMETERPOP_RUN_CONTROL: Address = 0x44198;
    pub const PARAMETERPOP_RUN_STATUS: Address = 0x441a8;
    pub const PARAMETERPOP_BREAK_POINT: Address = 0x441a0;
    pub const SCALARCORE_RUN_CONTROL: Address = 0x44018;
    pub const SCALARCORE_RUN_STATUS: Address = 0x44258;
    pub const SCALARCORE_BREAK_POINT: Address = 0x44020;
    pub const PREDICATE_REGISTER_FILE: Address = 0x44500;
    pub const SCALAR_REGISTER_FILE: Address = 0x44400;
    pub const SYNC_COUNTER_AVDATA_INFEED: Address = 0x44060;
    pub const SYNC_COUNTER_AVDATA_POP: Address = 0x44050;
    pub const SYNC_COUNTER_PARAMETER_INFEED: Address = 0x44068;
    pub const SYNC_COUNTER_PARAMETER_POP: Address = 0x44058;
    pub const SYNC_COUNTER_PRODUCER_A: Address = 0x44078;
    pub const SYNC_COUNTER_PRODUCER_B: Address = 0x44080;
    pub const SYNC_COUNTER_RING_OUTFEED: Address = 0x44088;
    pub const SYNC_COUNTER_SCALAR_INFEED: Address = 0x44070;
    pub const SYNC_COUNTER_SCALAR_PIPELINE: Address = 0x44090;
    pub const AVDATAPOP_OVERWRITE_MODE: Address = 0x44170;
    pub const AVDATAPOP_ENABLE_TRACING: Address = 0x44178;
    pub const AVDATAPOP_TRACE: Address = 0x442c0;
    pub const INFEED_OVERWRITE_MODE: Address = 0x441f0;
    pub const INFEED_ENABLE_TRACING: Address = 0x441f8;
    pub const INFEED_TRACE: Address = 0x44340;
    pub const OUTFEED_OVERWRITE_MODE: Address = 0x44230;
    pub const OUTFEED_ENABLE_TRACING: Address = 0x44238;
    pub const OUTFEED_TRACE: Address = 0x44380;
    pub const PARAMETERPOP_OVERWRITE_MODE: Address = 0x441b0;
    pub const PARAMETERPOP_ENABLE_TRACING: Address = 0x441b8;
    pub const PARAMETERPOP_TRACE: Address = 0x44300;
    pub const MESHBUS0_RUN_CONTROL: Address = 0x42250;
    pub const MESHBUS0_RUN_STATUS: Address = 0x42258;
    pub const MESHBUS0_BREAK_POINT: Address = 0x42260;
    pub const MESHBUS1_RUN_CONTROL: Address = 0x42298;
    pub const MESHBUS1_RUN_STATUS: Address = 0x422a0;
    pub const MESHBUS1_BREAK_POINT: Address = 0x422a8;
    pub const MESHBUS2_RUN_CONTROL: Address = 0x422e0;
    pub const MESHBUS2_RUN_STATUS: Address = 0x422e8;
    pub const MESHBUS2_BREAK_POINT: Address = 0x422f0;
    pub const MESHBUS3_RUN_CONTROL: Address = 0x42328;
    pub const MESHBUS3_RUN_STATUS: Address = 0x42330;
    pub const MESHBUS3_BREAK_POINT: Address = 0x42338;
    pub const NARROW_TO_WIDE_RUN_CONTROL: Address = 0x42150;
    pub const NARROW_TO_WIDE_RUN_STATUS: Address = 0x42158;
    pub const NARROW_TO_WIDE_BREAK_POINT: Address = 0x42160;
    pub const OP_RUN_CONTROL: Address = 0x420c0;
    pub const OP_RUN_STATUS: Address = 0x420e0;
    pub const OP_BREAK_POINT: Address = 0x420d0;
    pub const RINGBUSCONSUMER0_RUN_CONTROL: Address = 0x42190;
    pub const RINGBUSCONSUMER0_RUN_STATUS: Address = 0x42198;
    pub const RINGBUSCONSUMER0_BREAK_POINT: Address = 0x421a0;
    pub const RINGBUSCONSUMER1_RUN_CONTROL: Address = 0x421d0;
    pub const RINGBUSCONSUMER1_RUN_STATUS: Address = 0x421d8;
    pub const RINGBUSCONSUMER1_BREAK_POINT: Address = 0x421e0;
    pub const RINGBUSPRODUCER_RUN_CONTROL: Address = 0x42210;
    pub const RINGBUSPRODUCER_RUN_STATUS: Address = 0x42218;
    pub const RINGBUSPRODUCER_BREAK_POINT: Address = 0x42220;
    pub const WIDE_TO_NARROW_RUN_CONTROL: Address = 0x42110;
    pub const WIDE_TO_NARROW_RUN_STATUS: Address = 0x42118;
    pub const WIDE_TO_NARROW_BREAK_POINT: Address = 0x42120;
    pub const SYNC_COUNTER_AVDATA: Address = 0x42028;
    pub const SYNC_COUNTER_MESH_EAST_IN: Address = 0x42048;
    pub const SYNC_COUNTER_MESH_EAST_OUT: Address = 0x42068;
    pub const SYNC_COUNTER_MESH_NORTH_IN: Address = 0x42040;
    pub const SYNC_COUNTER_MESH_NORTH_OUT: Address = 0x42060;
    pub const SYNC_COUNTER_MESH_SOUTH_IN: Address = 0x42050;
    pub const SYNC_COUNTER_MESH_SOUTH_OUT: Address = 0x42070;
    pub const SYNC_COUNTER_MESH_WEST_IN: Address = 0x42058;
    pub const SYNC_COUNTER_MESH_WEST_OUT: Address = 0x42078;
    pub const SYNC_COUNTER_NARROW_TO_WIDE: Address = 0x42090;
    pub const SYNC_COUNTER_PARAMETERS: Address = 0x42030;
    pub const SYNC_COUNTER_PARTIAL_SUMS: Address = 0x42038;
    pub const SYNC_COUNTER_RING_PRODUCER_A: Address = 0x420b0;
    pub const SYNC_COUNTER_RING_PRODUCER_B: Address = 0x420b8;
    pub const SYNC_COUNTER_RING_READ_A: Address = 0x42098;
    pub const SYNC_COUNTER_RING_READ_B: Address = 0x420a0;
    pub const SYNC_COUNTER_RING_WRITE: Address = 0x420a8;
    pub const SYNC_COUNTER_WIDE_TO_NARROW: Address = 0x42080;
    pub const SYNC_COUNTER_WIDE_TO_SCALING: Address = 0x42088;
    pub const DMAMESHBUS0_OVERWRITE_MODE: Address = 0x42270;
    pub const DMAMESHBUS0_ENABLE_TRACING: Address = 0x42278;
    pub const DMAMESHBUS0_TRACE: Address = 0x42740;
    pub const DMAMESHBUS1_OVERWRITE_MODE: Address = 0x422b8;
    pub const DMAMESHBUS1_ENABLE_TRACING: Address = 0x422c0;
    pub const DMAMESHBUS1_TRACE: Address = 0x427c0;
    pub const DMAMESHBUS2_OVERWRITE_MODE: Address = 0x42300;
    pub const DMAMESHBUS2_ENABLE_TRACING: Address = 0x42308;
    pub const DMAMESHBUS2_TRACE: Address = 0x42840;
    pub const DMAMESHBUS3_OVERWRITE_MODE: Address = 0x42348;
    pub const DMAMESHBUS3_ENABLE_TRACING: Address = 0x42350;
    pub const DMAMESHBUS3_TRACE: Address = 0x428c0;
    pub const DMNARR_TO_WIDE_OVERWRITE_MODE: Address = 0x42168;
    pub const DMNARR_TO_WIDE_ENABLE_TRACING: Address = 0x42170;
    pub const DMNARR_TO_WIDE_TRACE: Address = 0x42600;
    pub const DMARINGBUSCONSUMER0_OVERWRITE_MODE: Address = 0x421a8;
    pub const DMARINGBUSCONSUMER0_ENABLE_TRACING: Address = 0x421b0;
    pub const DMARINGBUSCONSUMER0_TRACE: Address = 0x42640;
    pub const DMARINGBUSCONSUMER1_OVERWRITE_MODE: Address = 0x421e8;
    pub const DMARINGBUSCONSUMER1_ENABLE_TRACING: Address = 0x421f0;
    pub const DMARINGBUSCONSUMER1_TRACE: Address = 0x42680;
    pub const DMARINGBUSPRODUCER_OVERWRITE_MODE: Address = 0x42228;
    pub const DMARINGBUSPRODUCER_ENABLE_TRACING: Address = 0x42230;
    pub const DMARINGBUSPRODUCER_TRACE: Address = 0x426c0;
    pub const DMAWIDE_TO_NARR_OVERWRITE_MODE: Address = 0x42128;
    pub const DMAWIDE_TO_NARR_ENABLE_TRACING: Address = 0x42130;
    pub const DMAWIDE_TO_NARR_TRACE: Address = 0x42500;
    pub const OP_OVERWRITE_MODE: Address = 0x420e8;
    pub const OP_ENABLE_TRACING: Address = 0x420f0;
    pub const OP_TRACE: Address = 0x42400;
    pub const INSTRUCTION_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48010;
    pub const INSTRUCTION_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48018;
    pub const INSTRUCTION_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48020;
    pub const INSTRUCTION_INBOUND_QUEUE_FULL_COUNTER: Address = 0x48028;
    pub const INPUT_ACTV_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48030;
    pub const INPUT_ACTV_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48038;
    pub const INPUT_ACTV_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48040;
    pub const INPUT_ACTV_INBOUND_QUEUE_FULL_COUNTER: Address = 0x48048;
    pub const PARAM_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48050;
    pub const PARAM_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48058;
    pub const PARAM_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48060;
    pub const PARAM_INBOUND_QUEUE_FULL_COUNTER: Address = 0x48068;
    pub const OUTPUT_ACTV_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48070;
    pub const OUTPUT_ACTV_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48078;
    pub const OUTPUT_ACTV_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48080;
    pub const OUTPUT_ACTV_INBOUND_QUEUE_FULL_COUNTER: Address = 0x48088;
    pub const STATUS_BLOCK_WRITE_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48090;
    pub const STATUS_BLOCK_WRITE_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48098;
    pub const STATUS_BLOCK_WRITE_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x480a0;
    pub const STATUS_BLOCK_WRITE_INBOUND_QUEUE_FULL_COUNTER: Address = 0x480a8;
    pub const QUEUE_FETCH_INBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x480b0;
    pub const QUEUE_FETCH_INBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x480b8;
    pub const QUEUE_FETCH_INBOUND_QUEUE_INSERTION_COUNTER: Address = 0x480c0;
    pub const QUEUE_FETCH_INBOUND_QUEUE_FULL_COUNTER: Address = 0x480c8;
    pub const INSTRUCTION_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x480d0;
    pub const INSTRUCTION_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x480d8;
    pub const INSTRUCTION_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x480e0;
    pub const INSTRUCTION_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x480e8;
    pub const INPUT_ACTV_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x480f0;
    pub const INPUT_ACTV_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x480f8;
    pub const INPUT_ACTV_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48100;
    pub const INPUT_ACTV_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x48108;
    pub const PARAM_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48110;
    pub const PARAM_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48118;
    pub const PARAM_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48120;
    pub const PARAM_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x48128;
    pub const OUTPUT_ACTV_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48130;
    pub const OUTPUT_ACTV_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48138;
    pub const OUTPUT_ACTV_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48140;
    pub const OUTPUT_ACTV_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x48148;
    pub const STATUS_BLOCK_WRITE_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48150;
    pub const STATUS_BLOCK_WRITE_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48158;
    pub const STATUS_BLOCK_WRITE_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48160;
    pub const STATUS_BLOCK_WRITE_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x48168;
    pub const QUEUE_FETCH_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48170;
    pub const QUEUE_FETCH_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48178;
    pub const QUEUE_FETCH_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x48180;
    pub const QUEUE_FETCH_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x48188;
    pub const PAGE_TABLE_REQUEST_OUTBOUND_QUEUE_TOTAL_OCCUPANCY: Address = 0x48190;
    pub const PAGE_TABLE_REQUEST_OUTBOUND_QUEUE_THRESHOLD_COUNTER: Address = 0x48198;
    pub const PAGE_TABLE_REQUEST_OUTBOUND_QUEUE_INSERTION_COUNTER: Address = 0x481a0;
    pub const PAGE_TABLE_REQUEST_OUTBOUND_QUEUE_FULL_COUNTER: Address = 0x481a8;
    pub const READ_TRACKING_FIFO_TOTAL_OCCUPANCY: Address = 0x481b0;
    pub const READ_TRACKING_FIFO_THRESHOLD_COUNTER: Address = 0x481b8;
    pub const READ_TRACKING_FIFO_INSERTION_COUNTER: Address = 0x481c0;
    pub const READ_TRACKING_FIFO_FULL_COUNTER: Address = 0x481c8;
    pub const WRITE_TRACKING_FIFO_TOTAL_OCCUPANCY: Address = 0x481d0;
    pub const WRITE_TRACKING_FIFO_THRESHOLD_COUNTER: Address = 0x481d8;
    pub const WRITE_TRACKING_FIFO_INSERTION_COUNTER: Address = 0x481e0;
    pub const WRITE_TRACKING_FIFO_FULL_COUNTER: Address = 0x481e8;
    pub const READ_BUFFER_TOTAL_OCCUPANCY: Address = 0x481f0;
    pub const READ_BUFFER_THRESHOLD_COUNTER: Address = 0x481f8;
    pub const READ_BUFFER_INSERTION_COUNTER: Address = 0x48200;
    pub const READ_BUFFER_FULL_COUNTER: Address = 0x48208;
    pub const AXI_AW_CREDIT_SHIM_TOTAL_OCCUPANCY: Address = 0x48210;
    pub const AXI_AW_CREDIT_SHIM_THRESHOLD_COUNTER: Address = 0x48218;
    pub const AXI_AW_CREDIT_SHIM_INSERTION_COUNTER: Address = 0x48220;
    pub const AXI_AW_CREDIT_SHIM_FULL_COUNTER: Address = 0x48228;
    pub const AXI_AR_CREDIT_SHIM_TOTAL_OCCUPANCY: Address = 0x48230;
    pub const AXI_AR_CREDIT_SHIM_THRESHOLD_COUNTER: Address = 0x48238;
    pub const AXI_AR_CREDIT_SHIM_INSERTION_COUNTER: Address = 0x48240;
    pub const AXI_AR_CREDIT_SHIM_FULL_COUNTER: Address = 0x48248;
    pub const AXI_W_CREDIT_SHIM_TOTAL_OCCUPANCY: Address = 0x48250;
    pub const AXI_W_CREDIT_SHIM_THRESHOLD_COUNTER: Address = 0x48258;
    pub const AXI_W_CREDIT_SHIM_INSERTION_COUNTER: Address = 0x48260;
    pub const AXI_W_CREDIT_SHIM_FULL_COUNTER: Address = 0x48268;
    pub const INSTRUCTION_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48270;
    pub const INPUT_ACTV_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48278;
    pub const PARAM_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48280;
    pub const OUTPUT_ACTV_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48288;
    pub const STATUS_BLOCK_WRITE_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48290;
    pub const QUEUE_FETCH_INBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x48298;
    pub const INSTRUCTION_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482a0;
    pub const INPUT_ACTV_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482a8;
    pub const PARAM_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482b0;
    pub const OUTPUT_ACTV_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482b8;
    pub const STATUS_BLOCK_WRITE_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482c0;
    pub const QUEUE_FETCH_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482c8;
    pub const PAGE_TABLE_REQUEST_OUTBOUND_QUEUE_EMPTY_CYCLES_COUNT: Address = 0x482d0;
    pub const READ_TRACKING_FIFO_EMPTY_CYCLES_COUNT: Address = 0x482d8;
    pub const WRITE_TRACKING_FIFO_EMPTY_CYCLES_COUNT: Address = 0x482e0;
    pub const READ_BUFFER_EMPTY_CYCLES_COUNT: Address = 0x482e8;
    pub const READ_REQUEST_ARBITER_INSTRUCTION_REQUEST_CYCLES: Address = 0x482f0;
    pub const READ_REQUEST_ARBITER_INSTRUCTION_BLOCKED_CYCLES: Address = 0x482f8;
    pub const READ_REQUEST_ARBITER_INSTRUCTION_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x48300;
    pub const READ_REQUEST_ARBITER_INSTRUCTION_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x48308;
    pub const READ_REQUEST_ARBITER_INPUT_ACTV_REQUEST_CYCLES: Address = 0x48310;
    pub const READ_REQUEST_ARBITER_INPUT_ACTV_BLOCKED_CYCLES: Address = 0x48318;
    pub const READ_REQUEST_ARBITER_INPUT_ACTV_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x48320;
    pub const READ_REQUEST_ARBITER_INPUT_ACTV_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x48328;
    pub const READ_REQUEST_ARBITER_PARAM_REQUEST_CYCLES: Address = 0x48330;
    pub const READ_REQUEST_ARBITER_PARAM_BLOCKED_CYCLES: Address = 0x48338;
    pub const READ_REQUEST_ARBITER_PARAM_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x48340;
    pub const READ_REQUEST_ARBITER_PARAM_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x48348;
    pub const READ_REQUEST_ARBITER_QUEUE_FETCH_REQUEST_CYCLES: Address = 0x48350;
    pub const READ_REQUEST_ARBITER_QUEUE_FETCH_BLOCKED_CYCLES: Address = 0x48358;
    pub const READ_REQUEST_ARBITER_QUEUE_FETCH_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x48360;
    pub const READ_REQUEST_ARBITER_QUEUE_FETCH_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x48368;
    pub const READ_REQUEST_ARBITER_PAGE_TABLE_REQUEST_REQUEST_CYCLES: Address = 0x48370;
    pub const READ_REQUEST_ARBITER_PAGE_TABLE_REQUEST_BLOCKED_CYCLES: Address = 0x48378;
    pub const READ_REQUEST_ARBITER_PAGE_TABLE_REQUEST_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x48380;
    pub const READ_REQUEST_ARBITER_PAGE_TABLE_REQUEST_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x48388;
    pub const WRITE_REQUEST_ARBITER_OUTPUT_ACTV_REQUEST_CYCLES: Address = 0x48390;
    pub const WRITE_REQUEST_ARBITER_OUTPUT_ACTV_BLOCKED_CYCLES: Address = 0x48398;
    pub const WRITE_REQUEST_ARBITER_OUTPUT_ACTV_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x483a0;
    pub const WRITE_REQUEST_ARBITER_OUTPUT_ACTV_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x483a8;
    pub const WRITE_REQUEST_ARBITER_STATUS_BLOCK_WRITE_REQUEST_CYCLES: Address = 0x483b0;
    pub const WRITE_REQUEST_ARBITER_STATUS_BLOCK_WRITE_BLOCKED_CYCLES: Address = 0x483b8;
    pub const WRITE_REQUEST_ARBITER_STATUS_BLOCK_WRITE_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x483c0;
    pub const WRITE_REQUEST_ARBITER_STATUS_BLOCK_WRITE_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x483c8;
    pub const ADDRESS_TRANSLATION_ARBITER_INSTRUCTION_REQUEST_CYCLES: Address = 0x483d0;
    pub const ADDRESS_TRANSLATION_ARBITER_INSTRUCTION_BLOCKED_CYCLES: Address = 0x483d8;
    pub const ADDRESS_TRANSLATION_ARBITER_INSTRUCTION_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x483e0;
    pub const ADDRESS_TRANSLATION_ARBITER_INSTRUCTION_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x483e8;
    pub const ADDRESS_TRANSLATION_ARBITER_INPUT_ACTV_REQUEST_CYCLES: Address = 0x483f0;
    pub const ADDRESS_TRANSLATION_ARBITER_INPUT_ACTV_BLOCKED_CYCLES: Address = 0x483f8;
    pub const ADDRESS_TRANSLATION_ARBITER_INPUT_ACTV_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x48400;
    pub const ADDRESS_TRANSLATION_ARBITER_INPUT_ACTV_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x48408;
    pub const ADDRESS_TRANSLATION_ARBITER_PARAM_REQUEST_CYCLES: Address = 0x48410;
    pub const ADDRESS_TRANSLATION_ARBITER_PARAM_BLOCKED_CYCLES: Address = 0x48418;
    pub const ADDRESS_TRANSLATION_ARBITER_PARAM_BLOCKED_BY_ARBITRATION_CYCLES: Address = 0x48420;
    pub const ADDRESS_TRANSLATION_ARBITER_PARAM_CYCLES_BLOCKED_OVER_THRESHOLD: Address = 0x48428;
    pub const ADDRESS_TRANSLATION_ARBITER_STATUS_BLOCK_WRITE_REQUEST_CYCLES: Address = 0x48430;
    pub const ADDRESS_TRANSLATION_ARBITER_STATUS_BLOCK_WRITE_BLOCKED_CYCLES: Address = 0x48438;
    pub const ADDRESS_TRANSLATION_ARBITER_STATUS_BLOCK_WRITE_BLOCKED_BY_ARBITRATION_CYCLES:
        Address = 0x48440;
    pub const ADDRESS_TRANSLATION_ARBITER_STATUS_BLOCK_WRITE_CYCLES_BLOCKED_OVER_THRESHOLD:
        Address = 0x48448;
    pub const ADDRESS_TRANSLATION_ARBITER_OUTPUT_ACTV_REQUEST_CYCLES: Address = 0x48450;
    pub const ADDRESS_TRANSLATION_ARBITER_OUTPUT_ACTV_BLOCKED_CYCLES: Address = 0x48458;
    pub const ADDRESS_TRANSLATION_ARBITER_OUTPUT_ACTV_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x48460;
    pub const ADDRESS_TRANSLATION_ARBITER_OUTPUT_ACTV_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x48468;
    pub const ADDRESS_TRANSLATION_ARBITER_QUEUE_FETCH_REQUEST_CYCLES: Address = 0x48470;
    pub const ADDRESS_TRANSLATION_ARBITER_QUEUE_FETCH_BLOCKED_CYCLES: Address = 0x48478;
    pub const ADDRESS_TRANSLATION_ARBITER_QUEUE_FETCH_BLOCKED_BY_ARBITRATION_CYCLES: Address =
        0x48480;
    pub const ADDRESS_TRANSLATION_ARBITER_QUEUE_FETCH_CYCLES_BLOCKED_OVER_THRESHOLD: Address =
        0x48488;
    pub const ISSUED_INTERRUPT_COUNT: Address = 0x48490;
    pub const DATA_READ_16BYTE_COUNT: Address = 0x48498;
    pub const WAITING_FOR_TAG_CYCLES: Address = 0x484a0;
    pub const WAITING_FOR_AXI_CYCLES: Address = 0x484a8;
    pub const SIMPLE_TRANSLATIONS: Address = 0x484b0;
    pub const INSTRUCTION_CREDITS_PER_CYCLE_SUM: Address = 0x484c8;
    pub const INPUT_ACTV_CREDITS_PER_CYCLE_SUM: Address = 0x484d0;
    pub const PARAM_CREDITS_PER_CYCLE_SUM: Address = 0x484d8;
    pub const OUTPUT_ACTV_CREDITS_PER_CYCLE_SUM: Address = 0x484e0;
    pub const STATUS_BLOCK_WRITE_CREDITS_PER_CYCLE_SUM: Address = 0x484e8;
    pub const QUEUE_FETCH_CREDITS_PER_CYCLE_SUM: Address = 0x484f0;
    pub const PAGE_TABLE_REQUEST_CREDITS_PER_CYCLE_SUM: Address = 0x484f8;
    pub const OUTPUT_ACTV_QUEUE_CONTROL: Address = 0x48500;
    pub const OUTPUT_ACTV_QUEUE_STATUS: Address = 0x48508;
    pub const OUTPUT_ACTV_QUEUE_DESCRIPTOR_SIZE: Address = 0x48510;
    pub const OUTPUT_ACTV_QUEUE_MINIMUM_SIZE: Address = 0x48518;
    pub const OUTPUT_ACTV_QUEUE_MAXIMUM_SIZE: Address = 0x48520;
    pub const OUTPUT_ACTV_QUEUE_BASE: Address = 0x48528;
    pub const OUTPUT_ACTV_QUEUE_STATUS_BLOCK_BASE: Address = 0x48530;
    pub const OUTPUT_ACTV_QUEUE_SIZE: Address = 0x48538;
    pub const OUTPUT_ACTV_QUEUE_TAIL: Address = 0x48540;
    pub const OUTPUT_ACTV_QUEUE_FETCHED_HEAD: Address = 0x48548;
    pub const OUTPUT_ACTV_QUEUE_COMPLETED_HEAD: Address = 0x48550;
    pub const OUTPUT_ACTV_QUEUE_INT_CONTROL: Address = 0x48558;
    pub const OUTPUT_ACTV_QUEUE_INT_STATUS: Address = 0x48560;
    pub const INSTRUCTION_QUEUE_CONTROL: Address = 0x48568;
    pub const INSTRUCTION_QUEUE_STATUS: Address = 0x48570;
    pub const INSTRUCTION_QUEUE_DESCRIPTOR_SIZE: Address = 0x48578;
    pub const INSTRUCTION_QUEUE_MINIMUM_SIZE: Address = 0x48580;
    pub const INSTRUCTION_QUEUE_MAXIMUM_SIZE: Address = 0x48588;
    pub const INSTRUCTION_QUEUE_BASE: Address = 0x48590;
    pub const INSTRUCTION_QUEUE_STATUS_BLOCK_BASE: Address = 0x48598;
    pub const INSTRUCTION_QUEUE_SIZE: Address = 0x485a0;
    pub const INSTRUCTION_QUEUE_TAIL: Address = 0x485a8;
    pub const INSTRUCTION_QUEUE_FETCHED_HEAD: Address = 0x485b0;
    pub const INSTRUCTION_QUEUE_COMPLETED_HEAD: Address = 0x485b8;
    pub const INSTRUCTION_QUEUE_INT_CONTROL: Address = 0x485c0;
    pub const INSTRUCTION_QUEUE_INT_STATUS: Address = 0x485c8;
    pub const INPUT_ACTV_QUEUE_CONTROL: Address = 0x485d0;
    pub const INPUT_ACTV_QUEUE_STATUS: Address = 0x485d8;
    pub const INPUT_QUEUE_DESCRIPTOR_SIZE: Address = 0x485e0;
    pub const INPUT_ACTV_QUEUE_MINIMUM_SIZE: Address = 0x485e8;
    pub const INPUT_ACTV_QUEUE_MAXIMUM_SIZE: Address = 0x485f0;
    pub const INPUT_ACTV_QUEUE_BASE: Address = 0x485f8;
    pub const INPUT_ACTV_QUEUE_STATUS_BLOCK_BASE: Address = 0x48600;
    pub const INPUT_ACTV_QUEUE_SIZE: Address = 0x48608;
    pub const INPUT_ACTV_QUEUE_TAIL: Address = 0x48610;
    pub const INPUT_ACTV_QUEUE_FETCHED_HEAD: Address = 0x48618;
    pub const INPUT_ACTV_QUEUE_COMPLETED_HEAD: Address = 0x48620;
    pub const INPUT_ACTV_QUEUE_INT_CONTROL: Address = 0x48628;
    pub const INPUT_ACTV_QUEUE_INT_STATUS: Address = 0x48630;
    pub const PARAM_QUEUE_CONTROL: Address = 0x48638;
    pub const PARAM_QUEUE_STATUS: Address = 0x48640;
    pub const PARAM_QUEUE_DESCRIPTOR_SIZE: Address = 0x48648;
    pub const PARAM_QUEUE_MINIMUM_SIZE: Address = 0x48650;
    pub const PARAM_QUEUE_MAXIMUM_SIZE: Address = 0x48658;
    pub const PARAM_QUEUE_BASE: Address = 0x48660;
    pub const PARAM_QUEUE_STATUS_BLOCK_BASE: Address = 0x48668;
    pub const PARAM_QUEUE_SIZE: Address = 0x48670;
    pub const PARAM_QUEUE_TAIL: Address = 0x48678;
    pub const PARAM_QUEUE_FETCHED_HEAD: Address = 0x48680;
    pub const PARAM_QUEUE_COMPLETED_HEAD: Address = 0x48688;
    pub const PARAM_QUEUE_INT_CONTROL: Address = 0x48690;
    pub const PARAM_QUEUE_INT_STATUS: Address = 0x48698;
    pub const SC_HOST_INT_COUNT: Address = 0x486d0;
    pub const DMA_PAUSE: Address = 0x486d8;
    pub const DMA_PAUSED: Address = 0x486e0;
    pub const STATUS_BLOCK_UPDATE: Address = 0x486e8;
    pub const HIB_ERROR_STATUS: Address = 0x486f0;
    pub const HIB_ERROR_MASK: Address = 0x486f8;
    pub const HIB_FIRST_ERROR_STATUS: Address = 0x48700;
    pub const HIB_FIRST_ERROR_TIMESTAMP: Address = 0x48708;
    pub const HIB_INJECT_ERROR: Address = 0x48710;
    pub const READ_REQUEST_ARBITER: Address = 0x48718;
    pub const WRITE_REQUEST_ARBITER: Address = 0x48720;
    pub const ADDRESS_TRANSLATION_ARBITER: Address = 0x48728;
    pub const SENDER_QUEUE_THRESHOLD: Address = 0x48730;
    pub const PAGE_FAULT_ADDRESS: Address = 0x48738;
    pub const INSTRUCTION_CREDITS: Address = 0x48740;
    pub const INPUT_ACTV_CREDITS: Address = 0x48748;
    pub const PARAM_CREDITS: Address = 0x48750;
    pub const OUTPUT_ACTV_CREDITS: Address = 0x48758;
    pub const PAUSE_STATE: Address = 0x48760;
    pub const SNAPSHOT: Address = 0x48768;
    pub const IDLE_ASSERT: Address = 0x48770;
    pub const WIRE_INT_PENDING_BIT_ARRAY: Address = 0x48778;
    pub const TILECONFIG0: Address = 0x48788;
    pub const TILECONFIG1: Address = 0x48790;
    pub const TOPOLOGY: Address = 0x44000;
    pub const SC_MEMORY_CAPACITY: Address = 0x44008;
    pub const TILE_MEMORY_CAPACITY: Address = 0x44010;
    pub const SC_MEMORY_ACCESS: Address = 0x44040;
    pub const SC_MEMORY_DATA: Address = 0x44048;
    pub const TIMEOUT: Address = 0x44288;
    pub const ERROR_SCALARCORE: Address = 0x44260;
    pub const ERROR_MASK_SCALARCORE: Address = 0x44268;
    pub const ERROR_FORCE_SCALARCORE: Address = 0x44270;
    pub const ERROR_TIMESTAMP_SCALARCORE: Address = 0x44278;
    pub const ERROR_INFO_SCALARCORE: Address = 0x44280;
    pub const CURRENT_PC: Address = 0x44028;
    pub const EXECUTE_CONTROL: Address = 0x44038;
    pub const AVDATAPOP_START_CYCLE: Address = 0x44180;
    pub const AVDATAPOP_END_CYCLE: Address = 0x44188;
    pub const AVDATAPOP_PROGRAM_COUNTER: Address = 0x44190;
    pub const AVDATAPOP_TTU_STATE_REG_FILE: Address = 0x442a0;
    pub const PARAMETERPOP_START_CYCLE: Address = 0x441c0;
    pub const PARAMETERPOP_END_CYCLE: Address = 0x441c8;
    pub const PARAMETERPOP_PROGRAM_COUNTER: Address = 0x441d0;
    pub const PARAMETERPOP_TTU_STATE_REG_FILE: Address = 0x442e0;
    pub const INFEED_START_CYCLE: Address = 0x44200;
    pub const INFEED_END_CYCLE: Address = 0x44208;
    pub const INFEED_PROGRAM_COUNTER: Address = 0x44210;
    pub const INFEED_TTU_STATE_REG_FILE: Address = 0x44320;
    pub const OUTFEED_START_CYCLE: Address = 0x44240;
    pub const OUTFEED_END_CYCLE: Address = 0x44248;
    pub const OUTFEED_PROGRAM_COUNTER: Address = 0x44250;
    pub const OUTFEED_TTU_STATE_REG_FILE: Address = 0x44360;
    pub const TILEID: Address = 0x42000;
    pub const SCRATCHPAD: Address = 0x42008;
    pub const MEMORY_ACCESS: Address = 0x42010;
    pub const MEMORY_DATA: Address = 0x42018;
    pub const DEEP_SLEEP: Address = 0x42020;
    pub const POWER_SAVE_DATA: Address = 0x420c8;
    pub const STALL_COUNTER: Address = 0x420d8;
    pub const OP_START_CYCLE: Address = 0x420f8;
    pub const OP_END_CYCLE: Address = 0x42100;
    pub const OP_PROGRAM_COUNTER: Address = 0x42108;
    pub const DMAWIDE_TO_NARROW_START_CYCLE: Address = 0x42138;
    pub const DMAWIDE_TO_NARROW_END_CYCLE: Address = 0x42140;
    pub const DMAWIDE_TO_NARROW_PROGRAM_COUNTER: Address = 0x42148;
    pub const DMNARR_TO_WIDE_START_CYCLE: Address = 0x42178;
    pub const DMNARR_TO_WIDE_END_CYCLE: Address = 0x42180;
    pub const DMNARR_TO_WIDE_PROGRAM_COUNTER: Address = 0x42188;
    pub const DMARINGBUSCONSUMER0_START_CYCLE: Address = 0x421b8;
    pub const DMARINGBUSCONSUMER0_END_CYCLE: Address = 0x421c0;
    pub const DMARINGBUSCONSUMER0_PROGRAM_COUNTER: Address = 0x421c8;
    pub const DMARINGBUSCONSUMER1_START_CYCLE: Address = 0x421f8;
    pub const DMARINGBUSCONSUMER1_END_CYCLE: Address = 0x42200;
    pub const DMARINGBUSCONSUMER1_PROGRAM_COUNTER: Address = 0x42208;
    pub const DMARINGBUSPRODUCER_START_CYCLE: Address = 0x42238;
    pub const DMARINGBUSPRODUCER_END_CYCLE: Address = 0x42240;
    pub const DMARINGBUSPRODUCER_PROGRAM_COUNTER: Address = 0x42248;
    pub const DMAMESHBUS0_START_CYCLE: Address = 0x42280;
    pub const DMAMESHBUS0_END_CYCLE: Address = 0x42288;
    pub const DMAMESHBUS0_PROGRAM_COUNTER: Address = 0x42290;
    pub const DMAMESHBUS1_START_CYCLE: Address = 0x422c8;
    pub const DMAMESHBUS1_END_CYCLE: Address = 0x422d0;
    pub const DMAMESHBUS1_PROGRAM_COUNTER: Address = 0x422d8;
    pub const DMAMESHBUS2_START_CYCLE: Address = 0x42310;
    pub const DMAMESHBUS2_END_CYCLE: Address = 0x42318;
    pub const DMAMESHBUS2_PROGRAM_COUNTER: Address = 0x42320;
    pub const DMAMESHBUS3_START_CYCLE: Address = 0x42358;
    pub const DMAMESHBUS3_END_CYCLE: Address = 0x42360;
    pub const DMAMESHBUS3_PROGRAM_COUNTER: Address = 0x42368;
    pub const ERROR_TILE: Address = 0x42370;
    pub const ERROR_MASK_TILE: Address = 0x42378;
    pub const ERROR_FORCE_TILE: Address = 0x42380;
    pub const ERROR_TIMESTAMP_TILE: Address = 0x42388;
    pub const ERROR_INFO_TILE: Address = 0x42390;
    pub const TIMEOUT_TILE: Address = 0x42398;
    pub const OP_TTU_STATE_REG_FILE: Address = 0x423c0;
    pub const WIDE_TO_NARROW_TTU_STATE_REG_FILE: Address = 0x42480;
    pub const NARROW_TO_WIDE_TTU_STATE_REG_FILE: Address = 0x42580;
    pub const RINGBUSCONSUMER0_TTU_STATE_REG_FILE: Address = 0x42620;
    pub const RINGBUSCONSUMER1_TTU_STATE_REG_FILE: Address = 0x42660;
    pub const RINGBUSPRODUCER_TTU_STATE_REG_FILE: Address = 0x426a0;
    pub const MESHBUS0_TTU_STATE_REG_FILE: Address = 0x42700;
    pub const MESHBUS1_TTU_STATE_REG_FILE: Address = 0x42780;
    pub const MESHBUS2_TTU_STATE_REG_FILE: Address = 0x42800;
    pub const MESHBUS3_TTU_STATE_REG_FILE: Address = 0x42880;
    pub const PAGE_TABLE_SIZE: Address = 0x46000;
    pub const EXTENDED_TABLE: Address = 0x46008;
    pub const DMA_PAUSE_HIB_KERNEL: Address = 0x46050;
    pub const PAGE_TABLE_INIT: Address = 0x46078;
    pub const MSIX_TABLE_INIT: Address = 0x46080;
    pub const PAGE_TABLE: Address = 0x50000;
    pub const INSTRUCTION_QUEUE_INT_VECTOR: Address = 0x46018;
    pub const INPUT_ACTV_QUEUE_INT_VECTOR: Address = 0x46020;
    pub const PARAM_QUEUE_INT_VECTOR: Address = 0x46028;
    pub const OUTPUT_ACTV_QUEUE_INT_VECTOR: Address = 0x46030;
    pub const TOP_LEVEL_INT_VECTOR: Address = 0x46040;
    pub const SC_HOST_INT_VECTOR: Address = 0x46038;
    pub const FATAL_ERR_INT_VECTOR: Address = 0x46048;
    pub const MSIX_PENDING_BIT_ARRAY0: Address = 0x46068;
    pub const MSIX_TABLE: Address = 0x46800;
    pub const DMA_BURST_LIMITER_HIB_USER: Address = 0x487a8;
    pub const IDLE_REGISTER: Address = 0x4a000;
    pub const OP_RUN_CONTROL_TILE: Address = 0x400c0;
    pub const NARROW_TO_WIDE_RUN_CONTROL_TILE: Address = 0x40150;
    pub const WIDE_TO_NARROW_RUN_CONTROL_TILE: Address = 0x40110;
    pub const RINGBUSCONSUMER0_RUN_CONTROL_TILE: Address = 0x40190;
    pub const RINGBUSCONSUMER1_RUN_CONTROL_TILE: Address = 0x401d0;
    pub const RINGBUSPRODUCER_RUN_CONTROL_TILE: Address = 0x40210;
    pub const MESHBUS0_RUN_CONTROL_TILE: Address = 0x40250;
    pub const MESHBUS1_RUN_CONTROL_TILE: Address = 0x40298;
    pub const MESHBUS2_RUN_CONTROL_TILE: Address = 0x402e0;
    pub const MESHBUS3_RUN_CONTROL_TILE: Address = 0x40328;
    pub const DEEP_SLEEP_TILE: Address = 0x40020;
    pub const ERROR_TILE_TILE: Address = 0x40370;
    pub const ERROR_MASK_TILE_TILE: Address = 0x40378;
    pub const ERROR_FORCE_TILE_TILE: Address = 0x40380;
    pub const ERROR_TIMESTAMP_TILE_TILE: Address = 0x40388;
    pub const ERROR_INFO_TILE_TILE: Address = 0x40390;
    pub const WIRE_INT_MASK_ARRAY: Address = 0x48780;
    pub const FATAL_ERR_INT_CONTROL_USB: Address = 0x4c060;
    pub const FATAL_ERR_INT_STATUS_USB: Address = 0x4c068;
    pub const SC_HOST_INT_0_CONTROL: Address = 0x4c0b0;
    pub const SC_HOST_INT_0_STATUS: Address = 0x4c0b8;
    pub const SC_HOST_INT_1_CONTROL: Address = 0x4c0c8;
    pub const SC_HOST_INT_1_STATUS: Address = 0x4c0d0;
    pub const SC_HOST_INT_2_CONTROL: Address = 0x4c0e0;
    pub const SC_HOST_INT_2_STATUS: Address = 0x4c0e8;
    pub const SC_HOST_INT_3_CONTROL: Address = 0x4c0f8;
    pub const SC_HOST_INT_3_STATUS: Address = 0x4c100;
    pub const TOP_LEVEL_INT_0_CONTROL: Address = 0x4c070;
    pub const TOP_LEVEL_INT_0_STATUS: Address = 0x4c078;
    pub const TOP_LEVEL_INT_1_CONTROL: Address = 0x4c080;
    pub const TOP_LEVEL_INT_1_STATUS: Address = 0x4c088;
    pub const TOP_LEVEL_INT_2_CONTROL: Address = 0x4c090;
    pub const TOP_LEVEL_INT_2_STATUS: Address = 0x4c098;
    pub const TOP_LEVEL_INT_3_CONTROL: Address = 0x4c0a0;
    pub const TOP_LEVEL_INT_3_STATUS: Address = 0x4c0a8;
    pub const OMC0_00: Address = 0x1a000;
    pub const OMC0_D4: Address = 0x1a0d4;
    pub const OMC0_D8: Address = 0x1a0d8;
    pub const OMC0_DC: Address = 0x1a0dc;
    pub const MST_ABM_EN: Address = 0x1a600;
    pub const SLV_ABM_EN: Address = 0x1a500;
    pub const SLV_ERR_RESP_ISR_MASK: Address = 0x1a558;
    pub const MST_ERR_RESP_ISR_MASK: Address = 0x1a658;
    pub const MST_WR_ERR_RESP: Address = 0x1a640;
    pub const MST_RD_ERR_RESP: Address = 0x1a644;
    pub const SLV_WR_ERR_RESP: Address = 0x1a540;
    pub const SLV_RD_ERR_RESP: Address = 0x1a544;
    pub const RAMBIST_CTRL_1: Address = 0x1a704;
    pub const EFUSE_00: Address = 0x1a200;
    pub const BO0_FIFO_STATUS: Address = 0x19018;
    pub const BO1_FIFO_STATUS: Address = 0x1901c;
    pub const BO2_FIFO_STATUS: Address = 0x19020;
    pub const BO3_FIFO_STATUS: Address = 0x19024;
    pub const GCBB_CREDIT0: Address = 0x1907c;
    pub const SCU_CTRL_0: Address = 0x1a30c;
    pub const SCU_CTRL_1: Address = 0x1a310;
    pub const SCU_CTRL_2: Address = 0x1a314;
    pub const SCU_CTRL_3: Address = 0x1a318;
    pub const SCU_CTRL_4: Address = 0x1a31c;
    pub const SCU_CTRL_5: Address = 0x1a320;
    pub const SCU_CTR_6: Address = 0x1a32c;
    pub const SCU_CTR_7: Address = 0x1a33c;
    pub const OUTFEED_CHUNK_LENGTH: Address = 0x4c058;
    pub const DESCR_EP: Address = 0x4c148;
    pub const EP_STATUS_CREDIT: Address = 0x4c150;
    pub const MULTI_BO_EP: Address = 0x4c160;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Omc0D4 {
    pub register: u32,
}

impl Omc0D4 {
    pub const RESET: Self = Self { register: 0x1 };

    #[inline(always)]
    pub const fn new(register: u32) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u32) {
        self.register = value;
    }

    #[inline(always)]
    pub fn method_sel(&self) -> u32 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn set_method_sel(&mut self, value: u32) {
        self.register = (self.register & !(0x1 << 0)) | ((value & 0x1) << 0);
    }

    #[inline(always)]
    pub fn thm_warn1(&self) -> u32 {
        (self.register >> 16) & 0x3ff
    }

    #[inline(always)]
    pub fn set_thm_warn1(&mut self, value: u32) {
        self.register = (self.register & !(0x3ff << 16)) | ((value & 0x3ff) << 16);
    }

    #[inline(always)]
    pub fn thm_warn_en(&self) -> u32 {
        (self.register >> 31) & 0x1
    }

    #[inline(always)]
    pub fn set_thm_warn_en(&mut self, value: u32) {
        self.register = (self.register & !(0x1 << 31)) | ((value & 0x1) << 31);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Omc0D8 {
    pub register: u64,
}

impl Omc0D8 {
    pub const RESET: Self = Self { register: 0x0 };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn enbg(&self) -> u64 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn set_enbg(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 0)) | ((value & 0x1) << 0);
    }

    #[inline(always)]
    pub fn envr(&self) -> u64 {
        (self.register >> 1) & 0x1
    }

    #[inline(always)]
    pub fn set_envr(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 1)) | ((value & 0x1) << 1);
    }

    #[inline(always)]
    pub fn enad(&self) -> u64 {
        (self.register >> 2) & 0x1
    }

    #[inline(always)]
    pub fn set_enad(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 2)) | ((value & 0x1) << 2);
    }

    #[inline(always)]
    pub fn thm_warn2(&self) -> u64 {
        (self.register >> 16) & 0x3ff
    }

    #[inline(always)]
    pub fn set_thm_warn2(&mut self, value: u64) {
        self.register = (self.register & !(0x3ff << 16)) | ((value & 0x3ff) << 16);
    }

    #[inline(always)]
    pub fn sd_en(&self) -> u64 {
        (self.register >> 31) & 0x1
    }

    #[inline(always)]
    pub fn set_sd_en(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 31)) | ((value & 0x1) << 31);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Omc0DC {
    pub register: u64,
}

impl Omc0DC {
    pub const RESET: Self = Self {
        register: 0x3ff << 16,
    };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn data(&self) -> u64 {
        (self.register >> 16) & 0x3ff
    }

    #[inline(always)]
    pub fn set_data(&mut self, value: u64) {
        self.register = (self.register & !(0x3ff << 16)) | ((value & 0x3ff) << 16);
    }

    #[inline(always)]
    pub fn sd_clear(&self) -> u64 {
        (self.register >> 28) & 0x1
    }

    #[inline(always)]
    pub fn set_sd_clear(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 28)) | ((value & 0x1) << 28);
    }

    #[inline(always)]
    pub fn warn_clear(&self) -> u64 {
        (self.register >> 29) & 0x1
    }

    #[inline(always)]
    pub fn set_warn_clear(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 29)) | ((value & 0x1) << 29);
    }

    #[inline(always)]
    pub fn sd_o(&self) -> u64 {
        (self.register >> 30) & 0x1
    }

    #[inline(always)]
    pub fn warn_o(&self) -> u64 {
        (self.register >> 31) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RamBistCtrl1 {
    pub register: u64,
}

impl RamBistCtrl1 {
    pub const RESET: Self = Self {
        register: (0x1f << 0) | (0x3 << 5) | (0x7 << 20),
    };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn rg_rambist_gcbsel(&self) -> u64 {
        (self.register >> 0) & 0x1f
    }

    #[inline(always)]
    pub fn set_rg_rambist_gcbsel(&mut self, value: u64) {
        self.register = (self.register & !(0x1f << 0)) | ((value & 0x1f) << 0);
    }

    #[inline(always)]
    pub fn rg_rambist_topsel(&self) -> u64 {
        (self.register >> 5) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_rambist_topsel(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 5)) | ((value & 0x3) << 5);
    }

    #[inline(always)]
    pub fn rg_rambist_tckmode(&self) -> u64 {
        (self.register >> 8) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_rambist_tckmode(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 8)) | ((value & 0x1) << 8);
    }

    #[inline(always)]
    pub fn rg_rambist_req(&self) -> u64 {
        (self.register >> 9) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_rambist_req(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 9)) | ((value & 0x1) << 9);
    }

    #[inline(always)]
    pub fn rg_tck_invert(&self) -> u64 {
        (self.register >> 10) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_tck_invert(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 10)) | ((value & 0x1) << 10);
    }

    #[inline(always)]
    pub fn mbist_status(&self) -> u64 {
        (self.register >> 12) & 0x3
    }

    #[inline(always)]
    pub fn set_mbist_status(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 12)) | ((value & 0x3) << 12);
    }

    #[inline(always)]
    pub fn rg_mbist_int_status(&self) -> u64 {
        (self.register >> 16) & 0x7
    }

    #[inline(always)]
    pub fn set_rg_mbist_int_status(&mut self, value: u64) {
        self.register = (self.register & !(0x7 << 16)) | ((value & 0x7) << 16);
    }

    #[inline(always)]
    pub fn rg_mbist_int_mask(&self) -> u64 {
        (self.register >> 20) & 0x7
    }

    #[inline(always)]
    pub fn set_rg_mbist_int_mask(&mut self, value: u64) {
        self.register = (self.register & !(0x7 << 20)) | ((value & 0x7) << 20);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Efuse00 {
    pub register: u64,
}

impl Efuse00 {
    pub const RESET: Self = Self { register: 0x0 };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn ef_single_step_dis(&self) -> u64 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn ef_prod_sel(&self) -> u64 {
        (self.register >> 1) & 0x3
    }

    #[inline(always)]
    pub fn ef_refclk_sel_ovr(&self) -> u64 {
        (self.register >> 3) & 0x7
    }

    #[inline(always)]
    pub fn ef_pcie_gen1_link(&self) -> u64 {
        (self.register >> 6) & 0x1
    }

    #[inline(always)]
    pub fn ef_usb_ssc_mode_0(&self) -> u64 {
        (self.register >> 7) & 0x1
    }

    #[inline(always)]
    pub fn ef_i2caddr_ovr(&self) -> u64 {
        (self.register >> 8) & 0x1f
    }

    #[inline(always)]
    pub fn ef_psigma(&self) -> u64 {
        (self.register >> 13) & 0x7
    }

    #[inline(always)]
    pub fn ef_mbist_dis(&self) -> u64 {
        (self.register >> 16) & 0x1
    }

    #[inline(always)]
    pub fn ef_w_dis(&self) -> u64 {
        (self.register >> 17) & 0x1
    }

    #[inline(always)]
    pub fn ef_thm_int_mask(&self) -> u64 {
        (self.register >> 18) & 0x1
    }

    #[inline(always)]
    pub fn ef_int_mask(&self) -> u64 {
        (self.register >> 19) & 0x1
    }

    #[inline(always)]
    pub fn set_ef_int_mask(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 19)) | ((value & 0x1) << 19);
    }

    #[inline(always)]
    pub fn ef_pwr_state_dis(&self) -> u64 {
        (self.register >> 20) & 0x3
    }

    #[inline(always)]
    pub fn ef_usb_ssc_mode_1(&self) -> u64 {
        (self.register >> 22) & 0x1
    }

    #[inline(always)]
    pub fn ef_8051_rom_500m(&self) -> u64 {
        (self.register >> 23) & 0x1
    }

    #[inline(always)]
    pub fn ef_pll_m(&self) -> u64 {
        (self.register >> 24) & 0xff
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScuCtrl0 {
    pub register: u64,
}

impl ScuCtrl0 {
    pub const RESET: Self = Self {
        register: (0x1 << 0) | (0x1 << 6) | (0x1 << 8) | (0x1 << 11),
    };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn rg_pllclk_sel(&self) -> u64 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_pllclk_sel(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 0)) | ((value & 0x1) << 0);
    }

    #[inline(always)]
    pub fn rg_single_exit(&self) -> u64 {
        (self.register >> 1) & 0x1
    }

    #[inline(always)]
    pub fn rg_single_link_rstn(&self) -> u64 {
        (self.register >> 2) & 0x1
    }

    #[inline(always)]
    pub fn rg_sleep_chk_idle(&self) -> u64 {
        (self.register >> 3) & 0x1
    }

    #[inline(always)]
    pub fn rg_pcie_slp_phy_mode(&self) -> u64 {
        (self.register >> 4) & 0x3
    }

    #[inline(always)]
    pub fn rg_usb_slp_phy_mode(&self) -> u64 {
        (self.register >> 6) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_usb_slp_phy_mode(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 6)) | ((value & 0x3) << 6);
    }

    #[inline(always)]
    pub fn rg_pcie_inact_phy_mode(&self) -> u64 {
        (self.register >> 8) & 0x7
    }

    #[inline(always)]
    pub fn set_rg_pcie_inact_phy_mode(&mut self, value: u64) {
        self.register = (self.register & !(0x7 << 8)) | ((value & 0x7) << 8);
    }

    #[inline(always)]
    pub fn rg_usb_inact_phy_mode(&self) -> u64 {
        (self.register >> 11) & 0x7
    }

    #[inline(always)]
    pub fn set_rg_usb_inact_phy_mode(&mut self, value: u64) {
        self.register = (self.register & !(0x7 << 11)) | ((value & 0x7) << 11);
    }

    #[inline(always)]
    pub fn rg_mem_mode_dis(&self) -> u64 {
        (self.register >> 14) & 0x3
    }

    #[inline(always)]
    pub fn rg_phy_prg(&self) -> u64 {
        (self.register >> 16) & 0x1
    }

    #[inline(always)]
    pub fn bt_phy_prg(&self) -> u64 {
        (self.register >> 17) & 0x1
    }

    #[inline(always)]
    pub fn bt_vbus_sel(&self) -> u64 {
        (self.register >> 18) & 0x1
    }

    #[inline(always)]
    pub fn bt_bus_pwr(&self) -> u64 {
        (self.register >> 19) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScuCtrl2 {
    pub register: u64,
}

impl ScuCtrl2 {
    pub const RESET: Self = Self { register: 0x0 };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn rg_rst_pcie(&self) -> u64 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_pcie_axi(&self) -> u64 {
        (self.register >> 1) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_gcb(&self) -> u64 {
        (self.register >> 2) & 0x3
    }

    #[inline(always)]
    pub fn rg_rst_pcieslv_abm(&self) -> u64 {
        (self.register >> 4) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_pciemst_abm(&self) -> u64 {
        (self.register >> 5) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_omc(&self) -> u64 {
        (self.register >> 6) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_mbist(&self) -> u64 {
        (self.register >> 7) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_usb(&self) -> u64 {
        (self.register >> 8) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_usb_subsys(&self) -> u64 {
        (self.register >> 9) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_full(&self) -> u64 {
        (self.register >> 10) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_link(&self) -> u64 {
        (self.register >> 11) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_i2c(&self) -> u64 {
        (self.register >> 12) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_scu(&self) -> u64 {
        (self.register >> 13) & 0x1
    }

    #[inline(always)]
    pub fn rg_self_rst_subsys(&self) -> u64 {
        (self.register >> 14) & 0x1
    }

    #[inline(always)]
    pub fn rg_rst_brg(&self) -> u64 {
        (self.register >> 15) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_pcie(&self) -> u64 {
        (self.register >> 16) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_phy_cfg(&self) -> u64 {
        (self.register >> 17) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_gcb(&self) -> u64 {
        (self.register >> 18) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_gated_gcb(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 18)) | ((value & 0x3) << 18);
    }

    #[inline(always)]
    pub fn rg_gated_pcieslv_abm(&self) -> u64 {
        (self.register >> 20) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_pciemst_abm(&self) -> u64 {
        (self.register >> 21) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_omc(&self) -> u64 {
        (self.register >> 22) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_mbist(&self) -> u64 {
        (self.register >> 23) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_usb(&self) -> u64 {
        (self.register >> 24) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_usb_subsys(&self) -> u64 {
        (self.register >> 25) & 0x1
    }

    #[inline(always)]
    pub fn rg_gated_8051(&self) -> u64 {
        (self.register >> 26) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScuCtrl3 {
    pub register: u64,
}

impl ScuCtrl3 {
    pub const RESET: Self = Self {
        register: 0x80050410,
    };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn pcie_state_l1p2(&self) -> u64 {
        (self.register >> 0) & 0x1
    }

    #[inline(always)]
    pub fn pcie_state_l0s(&self) -> u64 {
        (self.register >> 1) & 0x1
    }

    #[inline(always)]
    pub fn pcie_state_l0(&self) -> u64 {
        (self.register >> 2) & 0x1
    }

    #[inline(always)]
    pub fn cur_gated_gcb(&self) -> u64 {
        (self.register >> 3) & 0x1
    }

    #[inline(always)]
    pub fn cur_rst_gcb(&self) -> u64 {
        (self.register >> 4) & 0x1
    }

    #[inline(always)]
    pub fn cur_gcb_sram_sd(&self) -> u64 {
        (self.register >> 6) & 0x1
    }

    #[inline(always)]
    pub fn cur_gcb_sram_dslp(&self) -> u64 {
        (self.register >> 7) & 0x1
    }

    #[inline(always)]
    pub fn cur_pwr_state(&self) -> u64 {
        (self.register >> 8) & 0x3
    }

    #[inline(always)]
    pub fn set_cur_pwr_state(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 8)) | ((value & 0x3) << 8);
    }

    #[inline(always)]
    pub fn pcie_gen_info(&self) -> u64 {
        (self.register >> 10) & 0x3
    }

    #[inline(always)]
    pub fn rg_force_ram_dslp(&self) -> u64 {
        (self.register >> 12) & 0x3
    }

    #[inline(always)]
    pub fn rg_force_ram_sd(&self) -> u64 {
        (self.register >> 14) & 0x3
    }

    #[inline(always)]
    pub fn rg_sd2wk_dly(&self) -> u64 {
        (self.register >> 16) & 0x7
    }

    #[inline(always)]
    pub fn rg_slp_mode_req(&self) -> u64 {
        (self.register >> 19) & 0x1
    }

    #[inline(always)]
    pub fn rg_force_inact(&self) -> u64 {
        (self.register >> 20) & 0x3
    }

    #[inline(always)]
    pub fn rg_force_sleep(&self) -> u64 {
        (self.register >> 22) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_force_sleep(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 22)) | ((value & 0x3) << 22);
    }

    #[inline(always)]
    pub fn rg_link_rdy_ovr(&self) -> u64 {
        (self.register >> 25) & 0x1
    }

    #[inline(always)]
    pub fn rg_pwr_state_ovr(&self) -> u64 {
        (self.register >> 26) & 0x3
    }

    #[inline(always)]
    pub fn rg_gcb_clkdiv(&self) -> u64 {
        (self.register >> 28) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_gcb_clkdiv(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 28)) | ((value & 0x3) << 28);
    }

    #[inline(always)]
    pub fn rg_axi_clk_125m(&self) -> u64 {
        (self.register >> 30) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_axi_clk_125m(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 30)) | ((value & 0x1) << 30);
    }

    #[inline(always)]
    pub fn rg_8051_clk_250m(&self) -> u64 {
        (self.register >> 31) & 0x1
    }

    #[inline(always)]
    pub fn set_rg_8051_clk_250m(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 31)) | ((value & 0x1) << 31);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScuCtrl6 {
    pub register: u64,
}

impl ScuCtrl6 {
    pub const RESET: Self = Self { register: 0x0 };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn rg_pad_ds(&self) -> u64 {
        (self.register >> 0) & 0x3
    }

    #[inline(always)]
    pub fn rg_pad_ds_i2c(&self) -> u64 {
        (self.register >> 2) & 0x3
    }

    #[inline(always)]
    pub fn rg_pad_ds_gpio(&self) -> u64 {
        (self.register >> 4) & 0x3
    }

    #[inline(always)]
    pub fn rg_pad_ds_xin(&self) -> u64 {
        (self.register >> 6) & 0x3
    }

    #[inline(always)]
    pub fn rg_pinmux_sel(&self) -> u64 {
        (self.register >> 8) & 0xff
    }

    #[inline(always)]
    pub fn rg_gcb_spare_in(&self) -> u64 {
        (self.register >> 16) & 0xf
    }

    #[inline(always)]
    pub fn set_rg_gcb_spare_in(&mut self, value: u64) {
        self.register = (self.register & !(0xf << 16)) | ((value & 0xf) << 16);
    }

    #[inline(always)]
    pub fn gcb_spare_out(&self) -> u64 {
        (self.register >> 20) & 0xf
    }

    #[inline(always)]
    pub fn warning_o(&self) -> u64 {
        (self.register >> 24) & 0x1
    }

    #[inline(always)]
    pub fn int_mbist(&self) -> u64 {
        (self.register >> 25) & 0x1
    }

    #[inline(always)]
    pub fn err_resp_isr_0(&self) -> u64 {
        (self.register >> 26) & 0x1
    }

    #[inline(always)]
    pub fn err_resp_isr_       _1(&self) -> u64 {
        (self.register >> 27) & 0x1
    }

    #[inline(always)]
    pub fn rg_jtag_sel(&self) -> u64 {
        (self.register >> 28) & 0x3
    }

    #[inline(always)]
    pub fn rg_jtag_io_sel(&self) -> u64 {
        (self.register >> 30) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScuCtrl7 {
    pub register: u64,
}

impl ScuCtrl7 {
    pub const RESET: Self = Self {
        register: (0x3f << 0) | (0x3 << 18),
    };

    #[inline(always)]
    pub const fn new(register: u64) -> Self {
        Self { register }
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.register
    }

    #[inline(always)]
    pub fn write(&mut self, value: u64) {
        self.register = value;
    }

    #[inline(always)]
    pub fn rg_inact_thd(&self) -> u64 {
        (self.register >> 0) & 0xffff
    }

    #[inline(always)]
    pub fn set_rg_inact_thd(&mut self, value: u64) {
        self.register = (self.register & !(0xffff << 0)) | ((value & 0xffff) << 0);
    }

    #[inline(always)]
    pub fn pll_lock_failure(&self) -> u64 {
        (self.register >> 16) & 0x1
    }

    #[inline(always)]
    pub fn set_pll_lock_failure(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 16)) | ((value & 0x1) << 16);
    }

    #[inline(always)]
    pub fn usb_sel_failure(&self) -> u64 {
        (self.register >> 17) & 0x1
    }

    #[inline(always)]
    pub fn set_usb_sel_failure(&mut self, value: u64) {
        self.register = (self.register & !(0x1 << 17)) | ((value & 0x1) << 17);
    }

    #[inline(always)]
    pub fn rg_boot_failure_mask(&self) -> u64 {
        (self.register >> 18) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_boot_failure_mask(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 18)) | ((value & 0x3) << 18);
    }

    #[inline(always)]
    pub fn rg_boot_failure_raw(&self) -> u64 {
        (self.register >> 20) & 0x3
    }

    #[inline(always)]
    pub fn set_rg_boot_failure_raw(&mut self, value: u64) {
        self.register = (self.register & !(0x3 << 20)) | ((value & 0x3) << 20);
    }
}
