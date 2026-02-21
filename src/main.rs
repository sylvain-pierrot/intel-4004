use intel_4004::bus::simple::SimpleBus;
use intel_4004::chips::{DataRam4002, Rom4001};
use intel_4004::dev::terminal::Terminal;
use intel_4004::machine::Machine;

fn main() {
    // let rom = Rom::from_bytes(&[
    //     0xD0, // LDM 0
    //     0xF2, // IAC
    //     0xF2, // IAC
    //     0x40, 0x01, // JUN 0x001
    // ]);
    let rom = Rom4001::from_bytes(&[
        // ---- demo: print "Hi" via RAM port (WMP) ----
        0xD4, 0xE1, 0xD8, 0xE1, // LDM 4, WMP | LDM 8, WMP  -> 'H' (0x48)
        0xD6, 0xE1, 0xD9, 0xE1, // LDM 6, WMP | LDM 9, WMP  -> 'i' (0x69)
        // ---- main ----
        0xD0, // 008: LDM 0
        0x50, 0x18, // 009: JMS 0x018 (sub)
        0x40, 0x28, // 00B: JUN 0x028
        // padding jusqu'à 0x018
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ---- SUB ----
        0xF2, // 018: IAC
        0xC0, // 019: BBL 0
        // padding jusqu'à 0x028
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ---- JUMP ----
        0xD5, // 028: LDM 5
    ]);
    let mut data = DataRam4002::default();
    data.attach_port(Terminal::new());
    let bus = SimpleBus::new(rom, data);

    let mut m = Machine::new(bus);

    m.run_steps(35);
    println!();
}
