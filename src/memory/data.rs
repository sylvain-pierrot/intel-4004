pub trait DataMemory {
    fn src(&mut self, addr: u8);
    fn dcl(&mut self, value: u8);
    fn read_character(&self) -> u8;
    fn read_status_character(&self, nbr: usize) -> u8;
    fn write_character(&mut self, value: u8);
    fn write_status_character(&mut self, nbr: usize, value: u8);
}

pub struct Register {
    characters: [u8; 16],
    status_characters: [u8; 4],
}

impl Register {
    pub fn new() -> Self {
        Self {
            characters: [0; 16],
            status_characters: [0; 4],
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DataRam4002 {
    pub banks: [[[Register; 4]; 4]; 8], // bank → chip → register
    addr8: u8,                          // latch d'adresse (SRC)
    bank: u8,                           // bank sélectionnée (DCL)
}

impl DataRam4002 {
    pub fn new() -> Self {
        Self {
            banks: std::array::from_fn(|_| {
                std::array::from_fn(|_| std::array::from_fn(|_| Register::default()))
            }),
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
}

impl DataMemory for DataRam4002 {
    fn src(&mut self, addr: u8) {
        self.addr8 = addr;
    }

    fn dcl(&mut self, value: u8) {
        self.bank = value & 0b0111;
    }

    fn read_character(&self) -> u8 {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[char]
    }

    fn read_status_character(&self, nbr: usize) -> u8 {
        let (chip, reg, _) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].status_characters[nbr]
    }

    fn write_character(&mut self, value: u8) {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[char] = value & 0xF;
    }

    fn write_status_character(&mut self, nbr: usize, value: u8) {
        let (chip, reg, _) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].status_characters[nbr] = value & 0xF;
    }
}

impl Default for DataRam4002 {
    fn default() -> Self {
        Self::new()
    }
}
