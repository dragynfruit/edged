
CSR: command status register
DMA: direct memory access
DarwiNN: There's a research paper on [efficient distributed neuroevolution under communication constraints](https://dl.acm.org/doi/10.1145/3377929.3390007), but I think it's just a code name here.

`driver::Registers`:
 - provides an interface for polling a given register by its offset
 - spins until getting expected value

`driver::SocketRegisters`:
 - provides an interface for sending requests and receiving results over OS sockets
 - IP and port number, so meant to be used over a network?

`driver::Bitfield`:
 - "Helper class to get/set command status register (CSR) fields"
 - does the stupid C++ operator definition thing

`driver::DeviceBufferMapper`:
 - "Maps request-specific Buffers to DeviceBuffers, and keeps track of DeviceBuffers."

`driver::DeviceBuffer`:
 - "Abstracts a device addressable buffer."

PCIe requires a kernel driver. USB does not.

