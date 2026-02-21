pub mod simple;

pub trait Bus {
    fn prog_read(&self, addr12: u16) -> u8;

    fn data_set_address(&mut self, addr8: u8);
    fn data_select_bank(&mut self, bank: u8);

    fn data_read(&self) -> u8;
    fn data_write(&mut self, value: u8);

    fn data_read_status(&self, idx: usize) -> u8;
    fn data_write_status(&mut self, idx: usize, value: u8);

    fn rom_port_write(&mut self, value: u8);
    fn rom_port_read(&mut self) -> u8;

    fn ram_port_write(&mut self, value: u8);
}
