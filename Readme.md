# `disarm64` definitions

This library provides structures and enums for describing AArch64 (ARMv8) instructions.
The approach might seem to be closer to what the `opcodes` library (a part of GNU `binutils`)
does, although the central motive is deserialization/parsing of a definition file -
what `TableGen` (of `LLVM`) employs.

For more information on the subject, please refer to:

- [LLVM & TableGen](https://github.com/llvm/llvm-project)
- [Binutils & libopcode](https://www.gnu.org/software/binutils/)
