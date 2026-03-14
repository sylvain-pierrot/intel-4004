// ── udp_hello — the 4004 sends a UDP datagram every second ───────────────────
//
// Listen in another terminal:
//   nc -u -l 1234
//
// Then run:
//   cargo run --example udp_hello

use intel_4004::bus::simple::SimpleBus;
use intel_4004::chips::{DataRam4002, Rom4001};
use intel_4004::dev::udp::UdpDevice;
use intel_4004::machine::Machine;
use std::time::Duration;

fn main() {
    let dev = UdpDevice::new("0.0.0.0:0", "127.0.0.1:1234")
        .expect("failed to bind UDP socket")
        .with_interval(Duration::from_secs(1));

    let mut rom = Rom4001::from_bytes(HELLO_ROM);
    rom.attach_port(dev);

    println!("4004 sending \"Hi, this is MCS-4\" → 127.0.0.1:1234  (Ctrl-C to stop)");
    println!("Listen with:  nc -u -l 1234");

    let bus = SimpleBus::new(rom, DataRam4002::default());
    let mut m = Machine::new(bus);
    m.run_until(|_| false);
}

// ── ROM ───────────────────────────────────────────────────────────────────────
//
// Protocol: first 2 WRR nibbles are the byte count (len_hi, len_lo).
// Then 2 nibbles per payload byte (high nibble first).
// The UdpDevice fires send() after the last byte.
//
// Message: "Hi, this is MCS-4\n"  →  18 bytes = 0x12
//
// Each byte becomes: LDM hi; WRR; LDM lo; WRR
//
// 000  D1 E2   len_hi = 1
// 002  D2 E2   len_lo = 2   → 0x12 = 18 bytes
// 004  D4 E2 D8 E2   'H' 0x48
// 008  D6 E2 D9 E2   'i' 0x69
// 00C  D2 E2 DC E2   ',' 0x2C
// 010  D2 E2 D0 E2   ' ' 0x20
// 014  D7 E2 D4 E2   't' 0x74
// 018  D6 E2 D8 E2   'h' 0x68
// 01C  D6 E2 D9 E2   'i' 0x69
// 020  D7 E2 D3 E2   's' 0x73
// 024  D2 E2 D0 E2   ' ' 0x20
// 028  D6 E2 D9 E2   'i' 0x69
// 02C  D7 E2 D3 E2   's' 0x73
// 030  D2 E2 D0 E2   ' ' 0x20
// 034  D4 E2 DD E2   'M' 0x4D
// 038  D4 E2 D3 E2   'C' 0x43
// 03C  D5 E2 D3 E2   'S' 0x53
// 040  D2 E2 DD E2   '-' 0x2D
// 044  D3 E2 D4 E2   '4' 0x34
// 048  D0 E2 DA E2   '\n' 0x0A
// 04C  40 00        JUN 000H  → loop

#[rustfmt::skip]
const HELLO_ROM: &[u8] = &[
    0xD1, 0xE2, 0xD2, 0xE2,  // len = 0x12 (18)
    0xD4, 0xE2, 0xD8, 0xE2,  // 'H' 0x48
    0xD6, 0xE2, 0xD9, 0xE2,  // 'i' 0x69
    0xD2, 0xE2, 0xDC, 0xE2,  // ',' 0x2C
    0xD2, 0xE2, 0xD0, 0xE2,  // ' ' 0x20
    0xD7, 0xE2, 0xD4, 0xE2,  // 't' 0x74
    0xD6, 0xE2, 0xD8, 0xE2,  // 'h' 0x68
    0xD6, 0xE2, 0xD9, 0xE2,  // 'i' 0x69
    0xD7, 0xE2, 0xD3, 0xE2,  // 's' 0x73
    0xD2, 0xE2, 0xD0, 0xE2,  // ' ' 0x20
    0xD6, 0xE2, 0xD9, 0xE2,  // 'i' 0x69
    0xD7, 0xE2, 0xD3, 0xE2,  // 's' 0x73
    0xD2, 0xE2, 0xD0, 0xE2,  // ' ' 0x20
    0xD4, 0xE2, 0xDD, 0xE2,  // 'M' 0x4D
    0xD4, 0xE2, 0xD3, 0xE2,  // 'C' 0x43
    0xD5, 0xE2, 0xD3, 0xE2,  // 'S' 0x53
    0xD2, 0xE2, 0xDD, 0xE2,  // '-' 0x2D
    0xD3, 0xE2, 0xD4, 0xE2,  // '4' 0x34
    0xD0, 0xE2, 0xDA, 0xE2,  // '\n' 0x0A
    0x40, 0x00,               // JUN 000H
];
