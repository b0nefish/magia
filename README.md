# Magia

Magia is an [Amiga](https://en.wikipedia.org/wiki/Amiga) emulator written in [Rust](https://www.rust-lang.org) that focus on [demos](https://en.wikipedia.org/wiki/Demoscene), accuracy and debuggability.
Magia is *not* a replacement for [UAE](https://fs-uae.net) but should be seen as an alternative for a specific niche which is the demoscene and demo development.
The emulator is built to make it easier for programmers to develop, inspect and debug demos/intros. If this seems unfamiliar to you [UAE](https://fs-uae.net) is likely a better choice for you.

# Status

This project is in very very early development. It can't currently do anything and won't likely be able to run demos/intros fully for a long time. Please be aware of that and that the structure of the code may change at any point.

# Code structure

## magia-api

(under design) The idea is to have an API (socket based) that allow people to get out data from the emulator to do their own visualisation, update memory, etc. This may be used by developers to implement "live loading" of data/code and such directly into memory.

## magia-core

Contains all the core parts such as chipset emulation, rendering to a screen buffer and such. It also collects all the data for debugging to be used by a front-end.

## magia-cli

Basic interface for magia which has a commandline debugger and a display (software) window

## magia-cpu

CPU emulation that is used by magia-core. The idea of having the CPU outside magia-core is that it can be used for other projects as well if needed. This code is based on currently based on https://github.com/marhel/r68k

## magia-disassembler

Disassembler for 68k code. Use by magia-core for the debugging interface

## Code of conduct

Any contribution to/participation in the Magia project is expected to follow the [Contributor Covenant code of conduct](code-of-conduct.md). Magia aims to be an open project where anyone can contribute, and upholding this code is essential to that goal.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

