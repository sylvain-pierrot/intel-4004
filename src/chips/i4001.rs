use crate::chips::Port;

pub struct Rom4001 {
    bytes: [u8; 4096],
    port: Port,
}

impl Rom4001 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut rom = [0u8; 4096];
        rom[..bytes.len().min(4096)].copy_from_slice(bytes);
        Self {
            bytes: rom,
            port: Port::default(),
        }
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Self::from_bytes(&data))
    }

    pub fn read_byte(&self, addr12: u16) -> u8 {
        self.bytes[(addr12 & 0x0FFF) as usize]
    }

    pub fn write_port(&mut self, value: u8) {
        self.port.write4(value);
    }

    pub fn read_port(&mut self) -> u8 {
        self.port.read4()
    }
}
