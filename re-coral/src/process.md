
## `libedgetpu`

This was by far the most time consuming and dull part.
I don't really have a background in C++, but the code seemed very convoluted.

...

## Reverse engineering `edgetpu_compiler`

`edgetpu_compiler` is actually where most of the interesting stuff happens and it's proprietary.

TensorFlow's tests include a set of Python scripts for generating valid models for each builtin operation.
This happens to be just what I needed.

According to Google, the following operations are (at least somewhat) supported[^1]:
 - Add
 - AveragePool2d
 - Concatenation
 - Conv2d
 - DepthwiseConv2d
 - ExpandDims
 - FullyConnected
 - L2Normalization
 - Logistic
 - LSTM
 - Maximum
 - MaxPool2d
 - Mean
 - Minimum
 - Mul
 - Pack
 - Pad
 - PReLU
 - Quantize
 - ReduceMax
 - ReduceMin
 - ReLU
 - ReLU6
 - ReLUN1To1
 - Reshape
 - ResizeBilinear
 - ResizeNearestNeighbor
 - Rsqrt
 - Slice
 - Softmax
 - SpaceToDepth
 - Split
 - Squeeze
 - StridedSlice
 - Sub
 - Sum
 - Squared-difference
 - Tanh
 - Transpose
 - TransposeConv

(Putting the entire list here seemed like a better idea before I typed it.)

So, I set up Bazel in a Github Codespace and let it run roughly five hours.
At which point it was unhappy with Ubuntu 22.04's clang and I decided to try again, twice.

After wasting a day and my little remaining sanity, I gave up and wrote some tooling myself in Python.
I did try Rust previously, but the maintainer of the broken Rust implementation of FlatBuffers isn't supporting it.

### First observations of the edgetpu

The instruction bitstreams are very interesting to me, so I took a look at them first.
These are just some random models I either had or downloaded from somewhere.

 > A generic MobileNet v2 model (min runtime 10):
 >
 > ```text
 > 00000000: 800f 0014 8400 0000 0000 0000 0000 0000  ................
 > 00000010: 0009 0000 9f01 0000 0000 0000 0000 0000  ................
 > 00000020: 0000 0000 0000 0000 0000 0000 0000 4003  ..............@.
 > 00000030: 0009 0000 0802 0000 0000 0000 0000 0000  ................
 > 00000040: 0000 0000 0000 0000 0000 0000 0000 4003  ..............@.
 > 00000050: 80f6 ff0f 00f0 ff1f 0080 ff01 0000 0000  ................
 > 00000060: 80f6 ff1f 0020 0080 0080 ff01 0000 0000  ..... ..........
 > 00000070: 0008 0000 0000 c01b 0000 0000 0000 0000  ................
 > 00000080: 0008 0000 0000 c00b 0000 0000 0000 0000  ................
 > 00000090: 0008 0000 0000 c03b 0000 0000 0000 0000  .......;........
 > 000000a0: 0008 0000 0000 c02b 0000 0000 0000 0000  .......+........
 > 000000b0: 80f6 ff2f 0000 0000 0080 ff01 0008 0000  .../............
 > 000000c0: 00f9 ff0f 0000 0002 00fe ff01 0000 0000  ................
 > 000000d0: 0000 0000 0000 0000 0000 0000 0000 0003  ................
 > 000000e0: c0f5 ff0f 00c0 0000 0000 0000 0000 0000  ................
 > 000000f0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
 > ```

 > A specialized MobileNet v2 model (min runtime 14):
 >
 > ```text
 > 00000000: 800f 00cc f100 0000 0000 0000 0000 0000  ................
 > 00000010: 80f6 ff0f 00f0 ff7f 0080 ff01 0008 0000  ................
 > 00000020: 00f9 ff0f ff01 0002 00fe ff01 0000 0000  ................
 > 00000030: 0000 0000 0000 0000 0000 0000 0000 c007  ................
 > 00000040: 0008 0000 0000 c01b 0000 0000 0000 0000  ................
 > 00000050: 0008 0000 0000 c00b 0000 0000 0000 0000  ................
 > 00000060: 0008 0000 0000 c03b 0000 0000 0000 0000  .......;........
 > 00000070: 0008 0000 0000 c02b 0000 0000 0000 0000  .......+........
 > 00000080: 0009 0000 0500 0000 0000 0000 0000 0000  ................
 > 00000090: 0000 0000 0000 0000 0000 0000 0000 0003  ................
 > 000000a0: 80f6 ff1f 0000 0000 0080 ff01 0008 0000  ................
 > 000000b0: 00f9 ff0f 0000 0002 00fe ff01 0000 0000  ................
 > 000000c0: 0000 0000 0000 0000 0000 0000 0000 0003  ................
 > 000000d0: c0f5 ff0f 0080 0000 0000 0000 0000 0000  ................
 > 000000e0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
 > 000000f0: 0010 0e08 0000 0000 0000 0000 0000 0000  ................
 > ```

 > A generic Yamnet model (min runtime 12):
 >
 > ```text
 > 00000000: 800f 0050 3200 0000 0000 0000 0000 0000  ...P2...........
 > 00000010: 80f6 ff0f 00f0 ff7f 0080 ff01 0008 0000  ................
 > 00000020: 00f9 ff0f ff01 0002 00fe ff01 0000 0000  ................
 > 00000030: 0000 0000 0000 0000 0000 0000 0000 c007  ................
 > 00000040: 0008 0000 0000 c01b 0000 0000 0000 0000  ................
 > 00000050: 0008 0000 0000 c00b 0000 0000 0000 0000  ................
 > 00000060: 0008 0000 0000 c03b 0000 0000 0000 0000  .......;........
 > 00000070: 0008 0000 0000 c02b 0000 0000 0000 0000  .......+........
 > 00000080: 0009 0000 0500 0000 0000 0000 0000 0000  ................
 > 00000090: 0000 0000 0000 0000 0000 0000 0000 0003  ................
 > 000000a0: 80f6 ff1f 0000 0000 0080 ff01 0008 0000  ................
 > 000000b0: 00f9 ff0f 0000 0002 00fe ff01 0000 0000  ................
 > 000000c0: 0000 0000 0000 0000 0000 0000 0000 0003  ................
 > 000000d0: c0f5 ff0f 0080 0000 0000 0000 0000 0000  ................
 > 000000e0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
 > 000000f0: 0000 0408 0000 0000 0000 0000 0000 0000  ................
 > ```

You may have noticed some patterns here.
All of them start with `0x800F`, followed by 4 bytes of data (or maybe 2 bytes of data w/ padding), followed by 10 null bytes.
`0x06` - `0xEF` are identical for the Yamnet model and specialized MobileNet v2 model, so I think `0x00` - `0xEF` are probably initialization stuff for those two.

Next, I spent a week trying to generate TFLite "models" to trick `edgetpu_compiler` into translating extremely simple single operations for me to compare and try to figure out.
It seems `edgetpu_compiler` didn't really like that.

```
Edge TPU Compiler version 16.0.384591198
Started a compilation timeout timer of 5 seconds.
Compilation child process completed within timeout period.
Compilation failed!
```

[^1]: <https://coral.ai/docs/edgetpu/models-intro/#supported-operations>
