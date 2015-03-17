# Titanos

Titanos is an exercise in writing a kernel in Rust programming language.

Immediate goal is to complete basic functionality targeting ARMv8 (aarch64)
and Vexpress board emulated by Qemu as a testing platform.

Everything is developed under Linux.

See [status page](//github.com/dpc/titanos/wiki/Status) for project status.

## Building

Follow `.travis.yml` to understand how to set up toolchain and external requirements.

* `make` builds everything
* `make run` to start the kernel inside Qemu
* `make debug` to start the kernel inside Qemu waiting for GDB connection
* `make gdb` to connect to Qemu instance started by `make debug`
* `make objdump` to dump assembler from the binary

To build in release mode, use `export RELEASE=1`.

## Design

Components:

* `src/`: source code
* `rt/`: basic runtime necessary for things to compile
* `c/`: some C code glue
* [titanium.rs][titanium]: Titanos is based on this
  collection of low-level macros, functions and constants that
  can be reused by other software targeting bare-metal development in Rust.
* [arm_pl011.rs][arm_pl011] - [Titanium.rs][titanium] based PL011 uart driver.

[titanium]: //github.com/dpc/titanium.rs
[arm_pl011]: //github.com/dpc/titanium_arm_pl011.rs

