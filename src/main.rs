pub mod cpu;
pub mod isa;
pub mod machine;
pub mod memory;

use crate::{
    machine::Machine,
    memory::{data::DataRam4002, program::Rom},
};

fn main() {
    let rom = Rom::from_bytes(&[
        0xD0, // LDM 0
        0xF2, // IAC
        0xF2, // IAC
        0x40, 0x01, // JUN 0x001
    ]);
    let data = DataRam4002::default();

    let mut machine = Machine::new(rom, data);

    machine.run();
}
