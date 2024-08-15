
# Edged

A driver for Coral Edge TPU devices, written in Rust

---

This is meant to be an alternative to `libedgetpu` for people that hate dealing with Google's bloated garbage software.
Edged has no dependency on TensorFlow, Flatbuffers, or Bazel whatsoever.

Our codebase is also a fraction of the size and doesn't have so much useless C++ complexity it's unmaintainable and needs to be reverse engineered.

## Features

 - WAY smaller codebase
 - No dependency on any specific ML framework
 - Better performance

## Usage

If you need M.2/PCIe support, install the Gasket kernel driver.

Then run this:
```shell
cargo add edged
```

---

This project is not owned or endorsed by Google in any way, shape, or form.
Trademarks belong to their respective rights holders.

