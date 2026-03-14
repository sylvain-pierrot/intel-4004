pub mod terminal;
pub mod udp;

pub trait IoDevice {
    fn write4(&mut self, nibble: u8);

    fn read4(&mut self) -> u8 {
        0
    }
}
