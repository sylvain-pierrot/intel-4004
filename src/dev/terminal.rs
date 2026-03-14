use std::io::Write;

use crate::dev::IoDevice;

#[derive(Default)]
pub struct Terminal {
    half: Option<u8>,
}

impl Terminal {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IoDevice for Terminal {
    fn write4(&mut self, value4: u8) {
        match self.half.take() {
            None => self.half = Some(value4 & 0xF),
            Some(hi) => {
                let byte = (hi << 4) | (value4 & 0xF);
                print!("{}", byte as char);
                let _ = std::io::stdout().flush();
            }
        }
    }
}
