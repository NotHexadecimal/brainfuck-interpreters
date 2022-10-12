# smol-bf

Fairly fast brainfuck interpreter written in rust with instruction grouping, fast loops and optional (on by default) JIT compiler based on GNU lightning.

Usage: <binary\> [-i <input-file\> | "input string"]

## Dependencies

If the jit feature is enabled you'll need GNU lightning installed on your system (lightning-devel on fedora)

## Architecture support

Guess it's the intersection of the archs supported by rust and those supported by lightning, only tested on amd64 though.
