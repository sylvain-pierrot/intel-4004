<h1 align="center">intel-4004</h1>

<p align="center">
    <em>Intel 4004 Microprocessor Emulator. Built in Silicon, Running in Rust.</em>
</p>
<p align="center">
    <img src="https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white" />
    <img src="https://img.shields.io/badge/Made_with_%E2%9D%A4_by-Sylvain_Pierrot-blueviolet?style=flat-square" />
</p>
<p align="center">
    <img width="400" src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/55/Intel_C4004.jpg/3840px-Intel_C4004.jpg" alt="Intel 4004 chip" />
</p>

---

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
  - [Why intel-4004?](#why-intel-4004)
- [Getting started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Program Examples](#program-examples)
- [📘 MCS-4 Architecture](#-mcs-4-architecture)
  - [Chips](#chips)
    - [CPU - Intel 4004](#cpu---intel-4004)
    - [ROM - Intel 4001](#rom---intel-4001)
    - [RAM - Intel 4002](#ram---intel-4002)
  - [Bus](#bus)
  - [I/O Devices](#io-devices)
- [📗 Instruction Set Reference](#-instruction-set-reference)
  - [Two-byte Instructions](#two-byte-instructions)
  - [One-byte Instructions](#one-byte-instructions)

## Overview

**intel-4004** is a cycle-accurate emulator of the Intel 4004, the world's first commercially available microprocessor (1971). It implements the full MCS-4 chipset: CPU, ROM, RAM, and I/O, faithfully replicating the original 4-bit architecture and instruction set.

> Based on the specifications defined in [Intel MCS-4 Assembly Language Programming Manual](https://bitsavers.org/components/intel/MCS4/MCS-4_Assembly_Language_Programming_Manual_Dec73.pdf).

### Why intel-4004?

This emulator was built to offer a faithful and extensible MCS-4 system that's:

- 🧮 **4-bit Architecture**: Authentic Intel 4004 emulation with 4-bit word size, 12-bit program counter, and 3-level call stack.
- ⚙️ **Full Instruction Set**: All 45 original 4004 instructions decoded and executed, including arithmetic, data movement, branching, and I/O.
- 🏗️ **MCS-4 Chipset**: Complete system emulation with Intel 4001 (ROM), Intel 4002 (RAM), and the Intel 4004 CPU, connected through a shared bus.
- 🔌 **Extensible I/O**: Abstract `IoDevice` trait to attach custom peripherals to ROM/RAM ports, with a terminal device included out of the box.
- 🐛 **Debug Tracing**: Optional `debug` feature flag enables verbose instruction-level tracing for step-by-step execution analysis.

---

## Getting started

### Prerequisites

This project requires the following dependencies:

- [Rust](https://www.rust-lang.org/tools/install) **>= 1.85** (2024 edition)
- `cargo` (bundled with Rust)

### Installation

Clone and build the project:

```bash
git clone https://github.com/sylvain-pierrot/intel-4004
cd intel-4004
cargo build --release
```

### Usage

Run the built-in demo program:

```bash
cargo run
```

Enable verbose instruction tracing with the `debug` feature:

```bash
cargo run --features debug
```

Use the library in your own project by adding it to `Cargo.toml`:

```toml
[dependencies]
intel-4004 = { path = "." }
```

Then assemble a program and run it:

```rust
use intel_4004::bus::simple::SimpleBus;
use intel_4004::chips::{DataRam4002, Rom4001};
use intel_4004::machine::Machine;

let rom = Rom4001::from_bytes(&[ /* your opcodes */ ]);
let ram = DataRam4002::default();
let bus = SimpleBus::new(rom, ram);

let mut machine = Machine::new(bus);
machine.run_steps(100);
```

### Program Examples

<details>
<summary><strong>Print "Hi" to the terminal via RAM port (WMP)</strong></summary>
<br>

The built-in demo writes two ASCII characters to the terminal by sending nibbles through the `WMP` (Write RAM Port) instruction. Each character is built from two consecutive 4-bit writes, which the terminal device assembles into a full byte.

```rust
let rom = Rom4001::from_bytes(&[
    // 'H' (0x48): high nibble 0x4, then low nibble 0x8
    0xD4, 0xE1,  // LDM 4 | WMP
    0xD8, 0xE1,  // LDM 8 | WMP
    // 'i' (0x69): high nibble 0x6, then low nibble 0x9
    0xD6, 0xE1,  // LDM 6 | WMP
    0xD9, 0xE1,  // LDM 9 | WMP
    // ...
]);
```

Output:

```
Hi
```

</details>

<details>
<summary><strong>Subroutine call with JMS / BBL</strong></summary>
<br>

The 4004 supports up to 3 levels of nested subroutine calls via its hardware stack. Use `JMS` to call a subroutine and `BBL` to return with a 4-bit value loaded into the accumulator.

```rust
let rom = Rom4001::from_bytes(&[
    // ---- main @ 0x008 ----
    0xD0,        // LDM 0        ; ACC = 0
    0x50, 0x18,  // JMS 0x018    ; call subroutine at 0x018
    0x40, 0x28,  // JUN 0x028    ; jump to 0x028
    // padding ...
    // ---- subroutine @ 0x018 ----
    0xF2,        // IAC          ; ACC = ACC + 1
    0xC0,        // BBL 0        ; return, ACC = 0
    // ...
]);
```

</details>

---

## 📘 MCS-4 Architecture

<p align="center">
    <img width="600" src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/87/4004_arch.svg/3840px-4004_arch.svg.png" alt="Intel 4004 architecture" />
</p>

The MCS-4 system is composed of discrete chips connected through a shared 4-bit bus. This emulator mirrors that physical layout using Rust structs and traits.

### Chips

#### CPU - Intel 4004

The `Cpu4004` is the processor core. It implements a full fetch/decode/execute cycle via the `step()` method.

| Register  | Size     | Description                               |
| --------- | -------- | ----------------------------------------- |
| `ACC`     | 4 bit    | Accumulator - primary arithmetic register |
| `CY`      | 1 bit    | Carry flag                                |
| `R[0-15]` | 4 bit    | 16 general-purpose registers (8 pairs)    |
| `PC`      | 12 bit   | Program counter                           |
| `Stack`   | 3x12 bit | 3-level hardware call stack               |
| `SP`      | 2 bit    | Stack pointer                             |

#### ROM - Intel 4001

`Rom4001` provides 4 KB (4096 bytes) of read-only program memory. It can be initialized from a byte slice or a file, and exposes a 4-bit I/O port for `WRR` / `RDR` instructions.

```rust
let rom = Rom4001::from_bytes(&[0xD5, 0xF2, /* ... */]);
```

#### RAM - Intel 4002

`DataRam4002` provides data memory organised in a hierarchical structure:

| Level      | Count | Description                         |
| ---------- | ----- | ----------------------------------- |
| Banks      | 8     | Selected by `DCL` instruction       |
| Chips      | 4     | Per bank                            |
| Registers  | 4     | Per chip (16 main + 4 status chars) |
| Characters | 16    | Per register (4-bit each)           |

The active address is latched by the `SRC` instruction. `DataRam4002` also exposes a 4-bit I/O port for the `WMP` instruction.

### Bus

`SimpleBus` connects the CPU to ROM and RAM, routing all memory and I/O operations through a single interface.

```
CPU4004  ──── SimpleBus ────┬─── Rom4001     (program memory + ROM port)
                            └─── DataRam4002 (data memory + RAM port)
```

Implement the `Bus` trait to create custom bus configurations.

### I/O Devices

The `IoDevice` trait can be implemented to attach peripherals to any chip port.

The bundled `Terminal` device accumulates pairs of 4-bit nibbles and prints the resulting ASCII character to stdout:

```rust
let mut ram = DataRam4002::default();
ram.attach_port(Terminal::new());
```

---

## 📗 Instruction Set Reference

The Intel 4004 has a 45-instruction set. All instructions are 1 byte wide, except those that encode a 12-bit or 8-bit address which require a second byte.

### Two-byte Instructions

| Mnemonic | Opcode     | Description                                              |
| -------- | ---------- | -------------------------------------------------------- |
| `JCN`    | `1X nn`    | Jump conditionally to 8-bit address based on `cond` mask |
| `FIM`    | `2P 00 nn` | Fetch immediate 8-bit value into register pair `P`       |
| `JUN`    | `4H LL`    | Unconditional jump to 12-bit address                     |
| `JMS`    | `5H LL`    | Jump to subroutine at 12-bit address (push PC to stack)  |
| `ISZ`    | `7R nn`    | Increment register `R`; jump to `nn` if result != 0      |

### One-byte Instructions

| Mnemonic | Opcode  | Description                                          |
| -------- | ------- | ---------------------------------------------------- |
| `NOP`    | `00`    | No operation                                         |
| `SRC`    | `2P 01` | Send register pair address to ROM/RAM                |
| `FIN`    | `3P 00` | Fetch indirect: load pair `P` from ROM using R0/R1   |
| `JIN`    | `3P 01` | Jump indirect to address in register pair `P`        |
| `INC`    | `6R`    | Increment register `R`                               |
| `ADD`    | `8R`    | Add register `R` to ACC with carry                   |
| `SUB`    | `9R`    | Subtract register `R` from ACC with borrow           |
| `LD`     | `AR`    | Load register `R` into ACC                           |
| `XCH`    | `BR`    | Exchange ACC with register `R`                       |
| `BBL`    | `Cn`    | Branch back and load: pop stack, load `n` into ACC   |
| `LDM`    | `Dn`    | Load immediate 4-bit value `n` into ACC              |
| `WRM`    | `E0`    | Write ACC to current RAM character                   |
| `WMP`    | `E1`    | Write ACC to RAM port (I/O)                          |
| `WRR`    | `E2`    | Write ACC to ROM port (I/O)                          |
| `WPM`    | `E3`    | Write ACC to program RAM                             |
| `WR0`    | `E4`    | Write ACC to RAM status character 0                  |
| `WR1`    | `E5`    | Write ACC to RAM status character 1                  |
| `WR2`    | `E6`    | Write ACC to RAM status character 2                  |
| `WR3`    | `E7`    | Write ACC to RAM status character 3                  |
| `SBM`    | `E8`    | Subtract RAM character from ACC with borrow          |
| `RDM`    | `E9`    | Read current RAM character into ACC                  |
| `RDR`    | `EA`    | Read ROM port into ACC                               |
| `ADM`    | `EB`    | Add RAM character to ACC with carry                  |
| `RD0`    | `EC`    | Read RAM status character 0 into ACC                 |
| `RD1`    | `ED`    | Read RAM status character 1 into ACC                 |
| `RD2`    | `EE`    | Read RAM status character 2 into ACC                 |
| `RD3`    | `EF`    | Read RAM status character 3 into ACC                 |
| `CLB`    | `F0`    | Clear both ACC and carry                             |
| `CLC`    | `F1`    | Clear carry                                          |
| `IAC`    | `F2`    | Increment ACC                                        |
| `CMC`    | `F3`    | Complement carry                                     |
| `CMA`    | `F4`    | Complement ACC                                       |
| `RAL`    | `F5`    | Rotate ACC left through carry                        |
| `RAR`    | `F6`    | Rotate ACC right through carry                       |
| `TCC`    | `F7`    | Transmit carry to ACC, then clear carry              |
| `DAC`    | `F8`    | Decrement ACC                                        |
| `TCS`    | `F9`    | Transfer carry subtract: load 9 or 10 into ACC       |
| `STC`    | `FA`    | Set carry                                            |
| `DAA`    | `FB`    | Decimal adjust ACC                                   |
| `KBP`    | `FC`    | Keyboard process: convert single active bit to index |
| `DCL`    | `FD`    | Designate command line: select RAM bank              |
