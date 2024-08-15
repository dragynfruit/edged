use std::{
    collections::{HashMap, VecDeque}, ops::{self, Bound, Index, Range, RangeBounds}, os::fd::RawFd, slice::SliceIndex
};

use crate::dma::{Address, DmaDirection};

#[derive(Clone, Copy)]
pub struct DeviceBuffer {
    size: usize,
    device_address: usize,
}
impl DeviceBuffer {
    pub fn new(device_address: usize, size: usize) -> Self {
        Self {
            device_address,
            size,
        }
    }

    pub fn slice<I: RangeBounds<usize>>(&self, index: I) -> Self {
        let st = match index.start_bound() {
            Bound::Unbounded => self.device_address,
            Bound::Included(&bound) => {
                assert!(bound < self.size);
                self.device_address + bound
            }
            Bound::Excluded(&bound) => {
                assert!(bound < self.size);
                self.device_address + bound - 1
            }
        };

        let ed = match index.end_bound() {
            Bound::Unbounded => self.device_address + self.size,
            Bound::Included(&bound) => {
                assert!(bound < self.size);
                self.device_address + bound
            }
            Bound::Excluded(&bound) => {
                assert!(bound <= self.size);
                self.device_address + bound - 1
            }
        };

        Self::new(st, ed - st)
    }
}
//impl<I: RangeBounds<usize>> Index<I> for DeviceBuffer {
//    type Output = DeviceBuffer;
//
//    #[inline]
//
//    fn index(&self, index: I) -> &Self::Output {
//        let st = match index.start_bound() {
//            Bound::Unbounded => {
//                self.device_address
//            },
//            Bound::Included(&bound) => {
//                assert!(bound < self.size);
//                self.device_address + bound
//            },
//            Bound::Excluded(&bound) => {
//                assert!(bound < self.size);
//                self.device_address + bound - 1
//            },
//        };
//
//        let ed = match index.end_bound() {
//            Bound::Unbounded => {
//                self.device_address + self.size
//            },
//            Bound::Included(&bound) => {
//                assert!(bound < self.size);
//                self.device_address + bound
//            },
//            Bound::Excluded(&bound) => {
//                assert!(bound <= self.size);
//                self.device_address + bound - 1
//            },
//        };
//
//        //&Self {
//        //    size: ed - st,
//        //    device_address: st,
//        //} as &Self
//        &Self::new(st, ed - st)
//    }
//}

pub enum BufKind {
    Invalid,
    // Wraps an existing host process addressable buffer.
    Wrapped,
    // Wraps an allocated host process addressable buffer.
    Allocated,
    // Wraps an mmap-able file descriptor, possibly from ION.
    FileDescriptor,
}
impl Default for BufKind {
    fn default() -> Self {
        Self::Invalid
    }
}

pub enum Buffer<'ca> {
    Invalid,
    Wrapped {
        len: usize,
    },
    Allocated {
        len: usize,
        vec: Vec<u8>,
    },
    FileDescriptor {
        len: usize,
        fd: RawFd,
    },
}
impl Buffer {
    pub fn alloc(start: Address, end: Address) -> Self {
        let len = (end - start) as usize;
        Self::Allocated { len: end-start as usize, vec: vec![0; len] }
    }

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        *self != Self::Invalid
    }
    #[inline]
    pub const fn len(&self) -> usize {
        match *self {
            Self::Invalid => 0,
            Self::Wrapped { len, .. } => len,
            Self::Allocated { len, .. } => len,
            Self::FileDescriptor { len, .. } => len,
        }
    }
    #[inline(always)]
    pub const fn is_mem_backed(&self) -> bool {
        *self == Self::Wrapped || *self == Self::Allocated
    }
    #[inline(always)]
    pub const fn is_fd_backed(&self) -> bool {
        *self == Self::FileDescriptor
    }
    #[inline(always)]
    pub const fn is_managed(&self) -> bool {
        *self == Self::Allocated
    }
}

pub type BufMap = HashMap<String, Vec<Buffer>>;
pub type DevBufMap = HashMap<String, Vec<DeviceBuffer>>;

pub struct DevBufferMapper {
    scratch: Option<DeviceBuffer>,

    inputs: NamedMap,
    outputs: NamedMap,

    input_mappings: Vec<DeviceBuffer>,
    output_mappings: Vec<DeviceBuffer>,

    instructions: Vec<DeviceBuffer>,

    instruction_mappings: Vec<DeviceBuffer>,
}
impl DevBufferMapper {
    pub fn map_multiple(&self, buffers: BufMap, dir: DmaDirection) -> (DevBufMap, Vec<DeviceBuffer>) {
        let mut user_bufs = DevBufMap::new();
        let mut mapped_bufs = Vec::new();

        // cleaner maybe
        
        // Separate the buffers into ptr- and non-ptr types.
  //std::vector<Buffer> ptr_buffers;

  let mut ptr_buffers = VecDeque::new();
  //for (const auto& name_and_buffer : buffers) {
      for (name, buffers) in buffers.iter() {
    //for (const auto& buffer : name_and_buffer.second) {
          for buf in buffers {
      if buf.is_mem_backed() {
        ptr_buffers.push_back(buf);
      }
          }
  }

  // Coalesce adjacent buffers. Since the underlying implementation can only map
  // whole pages, any buffers on the same page or adjacent pages can be merged
  // into a single underlying Map call. The basic algorithm is as follows:
  //
  // 1. Create a vector containing all start and end points, keeping a tag
  //    on each element indicating whether it was a start or end.
  // 2. Sort the vector, and if a start and end point have the same address, the
  //    start point should be first in sorted order.
  // 3. Iterate over the vector. Keep a running count of #start-#end points
  //    seen. Whenever this counter hits zero, that's the end of a merged
  //    interval.
  //
  // Because all the addresses are page-aligned, we can use the low bit to
  // distinguish between the start and end points.

  const END_OF_MAPPING_BIT: u64 = 1;

  //std::vector<uint64> addresses;
  let mut addresses: Vec<Address> = Vec::new();
  addresses.reserve(ptr_buffers.len() * 2);

  // merged_intervals contains the start address of each merged interval.
  // Pre-allocate space assuming that no merging will happen.
  //std::vector<uint8*> merged_intervals;
  let mut merged_intervals: Vec<[u8; 8]> = Vec::new();
  merged_intervals.reserve(ptr_buffers.size());

      for buffer in ptr_buffers {
    let start: u64 = GetPageAddress(reinterpret_cast<uintptr_t>(buffer.ptr()));
    let end: u64 =
        start +
        GetNumberPages(buffer.ptr(), buffer.size_bytes()) * kHostPageSize +
        END_OF_MAPPING_BIT;
    addresses.push_back(start);
    addresses.push_back(end);
  }

  std::sort(addresses.begin(), addresses.end());

  int count = 0;
  //for (uint64 address : addresses) {
  for addr in addresses {
    if addr & END_OF_MAPPING_BIT {
      count -= 1;
      assert!(count > 0);
      if count == 0 {
        let start = merged_intervals.pop();
        //uint8* end = reinterpret_cast<uint8*>(address - kEndOfMappingBit);
        let end = (addr - END_OF_MAPPING_BIT).to_le_bytes();
        Buffer merged_buffer(start, end - start);
        let merged_buffer = Buffer::
        //ASSIGN_OR_RETURN(auto device_buffer, Map(merged_buffer, direction));
        self.map(merged_buffer, dir)?;
        mapped_buffers.push_back(device_buffer);
      }
    } else {
      if (count == 0) {
        merged_intervals.push_back(reinterpret_cast<uint8*>(address));
      }
      ++count;
    }
  }

  // Figure out where the user's device buffers are within the merged buffers.
        for (name, buf) in buffers.iter() {
      DeviceBuffer device_buffer;
      if (buffer.IsPtrType()) {
        // Find the index of the corresponding merged buffer. In C++, there is
        // no way to directly binary search for an element that's less than a
        // given value, so instead we look for the closest one that's strictly
        // greater and subtract one from the index.
        const auto next = std::upper_bound(
            merged_intervals.begin(), merged_intervals.end(), buffer.ptr());
        int index = next - merged_intervals.begin() - 1;
        const auto merged = reinterpret_cast<uint8*>(merged_intervals[index]);
        const auto& mapped = mapped_buffers[index];
        device_buffer =
            DeviceBuffer(mapped.device_address() +
                         static_cast<uint64>(buffer.ptr() - merged),
                         buffer.size_bytes());
      } else {
        ASSIGN_OR_RETURN(device_buffer, Map(buffer, direction));
        mapped_buffers.push_back(device_buffer);
      }

      VLOG(3) << StringPrintf(
          "Mapped \"%s\" : %s -> 0x%016llx, %zu bytes. Direction=%d",
          name_and_buffer.first.c_str(), buffer.ToString().c_str(),
          static_cast<unsigned long long>(  // NOLINT(runtime/int)
              device_buffer.device_address()),
          device_buffer.size_bytes(), direction);

      user_buffers[name_and_buffer.first].push_back(std::move(device_buffer));
    }
  }

  cleaner.release();
  return OkStatus();
