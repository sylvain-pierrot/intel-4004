pub mod terminal;

pub trait IoDevice {
    fn write4(&mut self, value4: u8);

    fn read4(&mut self) -> u8 {
        0
    }
}
