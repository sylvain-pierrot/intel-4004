mod cli;

use msc_4::Msc4;
use msc_4::bus::simple::SimpleBus;
use msc_4::chips::{DataRam4002, Rom4001};
use msc_4::dev::terminal::Terminal;

use clap::Parser;

fn demo_rom() -> Vec<u8> {
    vec![
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
    ]
}

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();

    let rom = match cli.rom {
        Some(path) => Rom4001::from_file(path)?,
        None => Rom4001::from_bytes(&demo_rom()),
    };

    let mut data = DataRam4002::default();
    if cli::wants_terminal(&cli.devices) {
        data.attach_port(Terminal::new());
    }
    let bus = SimpleBus::new(rom, data);

    let mut msc_4 = Msc4::new(bus);
    msc_4.run_steps(cli.steps);

    Ok(())
}
