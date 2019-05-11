# Magia

Magia is an Amiga emulator written in Rust that focus on demos, accuracy and debuggability.
Magia is *not* a replacement for UAE but should be seen as an alternative for a specific niche which is the demoscene and demo development.
The emulator is built to make it easier for programmers to develop, inspect and debug demos/intros. If this seems unfamiliar to you UAE is likely a better choice for you.

# Code structure

## magia-core

Contains all the core parts such as chipset emulation, rendering to a screen buffer and such. It also collects all the data for debugging to be used by a front-end.

## magia-cpu

CPU emulation that is used by magia-core. The idea of having the CPU outside magia-core is that it can be used for other projects as well if needed.

## magia-cli

Basic interface for magia which has a commandline debugger and a display (software) window

## magia-api

(under design) The idea is to have an API (socket based) that allow people to get out data from the emulator to do their own visualisation, update memory, etc. This may be used by developers to implement "live loading" of data/code and such directly into memory.

