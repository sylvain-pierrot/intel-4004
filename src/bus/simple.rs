use crate::bus::Bus;
use crate::chips::{DataRam4002, Rom4001};

pub struct SimpleBus {
    pub prog: Rom4001,
    pub data: DataRam4002,
}

impl SimpleBus {
    pub fn new(prog: Rom4001, data: DataRam4002) -> Self {
        Self { prog, data }
    }
}

impl Bus for SimpleBus {
    fn prog_read(&self, addr12: u16) -> u8 {
        self.prog.read_byte(addr12 & 0x0FFF)
    }

    fn data_set_address(&mut self, addr8: u8) {
        self.data.set_address(addr8);
    }
    fn data_select_bank(&mut self, bank: u8) {
        self.data.select_bank(bank & 0b111);
    }

    fn data_read(&self) -> u8 {
        self.data.read() & 0xF
    }
    fn data_write(&mut self, value: u8) {
        self.data.write(value & 0xF);
    }

    fn data_read_status(&self, idx: usize) -> u8 {
        self.data.read_status(idx) & 0xF
    }
    fn data_write_status(&mut self, idx: usize, value: u8) {
        self.data.write_status(idx, value & 0xF);
    }

    fn rom_port_write(&mut self, value: u8) {
        self.prog.write_port(value & 0xF);
    }

    fn rom_port_read(&mut self) -> u8 {
        self.prog.read_port() & 0xF
    }

    fn ram_port_write(&mut self, value: u8) {
        self.data.write_port(value & 0xF);
    }
}
