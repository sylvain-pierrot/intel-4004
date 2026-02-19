pub mod data_ram;

pub trait Memory {
    fn src(&mut self, addr: u8);
    fn dcl(&mut self, value: u8);
    fn read_character(&self) -> u8;
    fn read_status_character(&self, nbr: usize) -> u8;
    fn write_character(&mut self, value: u8);
    fn write_status_character(&mut self, nbr: usize, value: u8);
}
