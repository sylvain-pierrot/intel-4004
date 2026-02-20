use intel_4004::machine::Machine;
use intel_4004::memory::{data::DataRam4002, program::Rom};

fn main() {
    // let rom = Rom::from_bytes(&[
    //     0xD0, // LDM 0
    //     0xF2, // IAC
    //     0xF2, // IAC
    //     0x40, 0x01, // JUN 0x001
    // ]);
    let rom = Rom::from_bytes(&[
        // ---- main ----
        0xD0, // 000: LDM 0
        0x50, 0x10, // 001: JMS 0x010
        0x40, 0x20, // 003: JUN 0x020
        // padding jusqu'à 0x010
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ---- SUB ----
        0xF2, // 010: IAC
        0xC0, // 011: BBL 0
        // padding jusqu'à 0x020
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ---- JUMP ----
        0xD5, // 020: LDM 5
    ]);
    let data = DataRam4002::default();

    let mut machine = Machine::new(rom, data);

    machine.run_steps(25);
}
