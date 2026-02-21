use crate::dev::IoDevice;

pub struct Terminal {
    half: Option<u8>,
}

impl Terminal {
    pub fn new() -> Self {
        Self { half: None }
    }
}

impl IoDevice for Terminal {
    fn write4(&mut self, value4: u8) {
        let n = value4 & 0x0F;
        match self.half.take() {
            None => self.half = Some(n),
            Some(hi) => {
                let byte = (hi << 4) | n;
                print!("{}", byte as char);
            }
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}
