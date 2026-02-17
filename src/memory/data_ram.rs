pub struct DataRam {
    pub banks: [[[[u8; 16]; 4]; 4]; 8], // bank → chip → register → char
    addr8: u8,                          // latch d'adresse (SRC)
    bank: usize,                        // bank sélectionnée (DCL)
}

impl DataRam {
    pub fn new() -> Self {
        Self {
            banks: [[[[0; 16]; 4]; 4]; 8],
            addr8: 0,
            bank: 0,
        }
    }

    fn decode_addr8(&self) -> (usize, usize, usize) {
        let chip = ((self.addr8 >> 6) & 0x3) as usize;
        let reg = ((self.addr8 >> 4) & 0x3) as usize;
        let char = (self.addr8 & 0xF) as usize;
        (chip, reg, char)
    }

    pub fn src(&mut self, addr: u8) {
        self.addr8 = addr;
    }

    pub fn dcl(&mut self, bank: usize) {
        self.bank = bank & 0b0111;
    }

    pub fn read_main(&self) -> u8 {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank][chip][reg][char]
    }

    pub fn write_main(&mut self, value: u8) {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank][chip][reg][char] = value & 0xF;
    }
}

impl Default for DataRam {
    fn default() -> Self {
        Self::new()
    }
}
