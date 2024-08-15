#![allow(nonstandard_style)]


use nix::{
    ioctl_none, ioctl_read, ioctl_readwrite, ioctl_write_int, ioctl_write_ptr,
};

#[repr(C)]
pub struct gasket_interrupt_eventfd {
    pub interrupt: u64,
    pub event_fd: u64,
}

#[repr(C)]
pub struct gasket_page_table {
    pub pt_index: u64,
    pub size: u64,
    pub host_addr: u64,
    pub dev_addr: u64,
}

#[repr(C)]
pub struct gasket_page_table_flags {
    pub base: gasket_page_table,
    pub flags: u32,
}

#[repr(C)]
pub struct gasket_config_coherent_alloc {
    pub pt_index: u64,
    pub enable: u64,
    pub size: u64,
    pub dma_addr: u64,
}

#[repr(C)]
pub struct gasket_page_table_dmabuf {
    pub pt_index: u64,
    pub dev_addr: u64,
    pub dmabuf_fd: i32,
    pub num_pages: u32,
    pub map: u32,
    pub flags: u32,
}

#[rustfmt::skip]
ioctl_none!(gasket_reset, 0xDC, 0);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_set_eventfd, 0xDC, 1, gasket_interrupt_eventfd);
#[rustfmt::skip]
ioctl_write_int!(gasket_clear_eventfd, 0xDC, 2);
#[rustfmt::skip]
ioctl_write_int!(gasket_loopback_interrupt, 0xDC, 3);
#[rustfmt::skip]
ioctl_read!(gasket_number_page_tables, 0xDC, 4, u64);
#[rustfmt::skip]
ioctl_readwrite!(gasket_page_table_size, 0xDC, 5, gasket_page_table);
#[rustfmt::skip]
ioctl_readwrite!(gasket_simple_page_table_size, 0xDC, 6, gasket_page_table);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_partition_page_table, 0xDC, 7, gasket_page_table);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_map_buffer, 0xDC, 8, gasket_page_table);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_unmap_buffer, 0xDC, 9, gasket_page_table);
#[rustfmt::skip]
ioctl_none!(gasket_clear_interrupt_counts, 0xDC, 10);
#[rustfmt::skip]
ioctl_readwrite!(gasket_config_coherent_allocator, 0xDC, 11, gasket_config_coherent_alloc);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_map_buffer_flags, 0xDC, 12, gasket_page_table_flags);
#[rustfmt::skip]
ioctl_write_ptr!(gasket_map_dmabuf, 0xDC, 13, gasket_page_table_dmabuf);
