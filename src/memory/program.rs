pub trait ProgramMemory {
    fn read_byte(&self, addr12: u16) -> u8;
}

pub struct Rom {
    bytes: [u8; 4096],
}

impl Rom {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut rom = [0u8; 4096];
        rom[..bytes.len().min(4096)].copy_from_slice(bytes);
        Self { bytes: rom }
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Self::from_bytes(&data))
    }
}

impl ProgramMemory for Rom {
    fn read_byte(&self, addr12: u16) -> u8 {
        self.bytes[(addr12 & 0x0FFF) as usize]
    }
}
