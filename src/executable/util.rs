use flatbuffers::Vector;

use crate::{dma::Address, flatbuffers::executable::platforms::darwinn::{Description, FieldOffset, Position}};

pub fn align_next(val: u32, n: u32) -> u32 {
    assert_eq!(0, n & (n-1));
    (val + n) & !(n-1)
}

pub fn copy_uint32(buf: &mut [u8], offset_bit: usize, orig_val: u32) {
    // Track current destination bit offset.
let mut next_dst_offset_bit = offset_bit;

  // Tracks remaining bits that needs to be set.
  let mut remaining_bits = u32::BITS;

  // Value that needs to be set, bits that are set are shifted out.
  let mut next_value = orig_val;

  while remaining_bits > 0 {
    // Sets enough bits to align to the next 8 bit boundary.
    let num_bits_to_set =
        (align_next(next_dst_offset_bit as u32, 8) - next_dst_offset_bit as u32).min(
                 remaining_bits);

    // Offset byte and bit offset with in the byte.
    let dst_byte = next_dst_offset_bit / 8;
    let dst_bit = next_dst_offset_bit % 8;

    // Copy lower |num_bits_to_set| from next_value into the destination byte
    // at the specified offset.
    //next_value = CopyUint8LowBits(next_value, dst_bit, num_bits_to_set,
    //                              &buf[dst_byte]);

    remaining_bits -= num_bits_to_set;
    next_dst_offset_bit += num_bits_to_set as usize;
  }
}

fn link_scratch_param_address(addr: Address, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {

    if field_offsets.is_empty() {
        return;
    }

    for fo in field_offsets {
        let meta = fo.meta().expect("UB: executable_util.cc:148");
        if meta.desc() != Description::BASE_ADDRESS_SCRATCH {
            continue;
        }

        assert_eq!(meta.batch(), 0);

        let immediate_val: u32;

        if meta.position() == Position::LOWER_32BIT {
            immediate_val = *addr as u32;
        } else {
            assert_eq!(meta.position(), Position::UPPER_32BIT);
            immediate_val = (addr >> 32) as u32;
        }

        copy_uint32(enc_buf, fo.offset_bit() as usize, immediate_val);
    }
}

fn link_batched_address(target: Description, name: &'static str, addresses: Vec<Address>, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {
    if field_offsets.is_empty() {
        return;
    }

    for fo in field_offsets {
        let meta = fo.meta().expect("UB: executable_util.cc:78");
        if meta.desc() != target || meta.name().expect("UB: executable_util.cc:83") != name {
            continue;
        }

        let batch = meta.batch() as usize;
        assert!(batch < addresses.len());
        let link_addr = addresses[batch];

        let immediate_val: u32;

        if meta.position() == Position::LOWER_32BIT {
            immediate_val = *link_addr as u32;
        } else {
            assert_eq!(meta.position(), Position::UPPER_32BIT);
            immediate_val = (link_addr >> 32) as u32;
        }

        copy_uint32(enc_buf, fo.offset_bit() as usize, immediate_val);
    }
}

pub fn link_scratch_address(addr: Address, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {
    link_scratch_param_address(addr, field_offsets, enc_buf);
}

pub fn link_param_address(addr: Address, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {
    link_scratch_param_address(addr, field_offsets, enc_buf);
}

pub fn link_input_address(name: &'static str, addresses: Vec<Address>, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {
    link_batched_address(Description::BASE_ADDRESS_INPUT_ACTIVATION, name, addresses, field_offsets, enc_buf)
}

pub fn link_output_address(name: &'static str, addresses: Vec<Address>, field_offsets: Vector<FieldOffset>, enc_buf: &mut [u8]) {
    link_batched_address(Description::BASE_ADDRESS_OUTPUT_ACTIVATION, name, addresses, field_offsets, enc_buf)
}
