use crate::chips::Port;
pub struct Register {
    characters: [u8; 16],
    status_characters: [u8; 4],
}

impl Register {
    fn new() -> Self {
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
    banks: [[[Register; 4]; 4]; 8], // bank → chip → register
    addr8: u8,                      // latch d'adresse (SRC)
    bank: u8,                       // bank sélectionnée (DCL)
    port: Port,
}

impl DataRam4002 {
    pub fn new() -> Self {
        Self {
            banks: std::array::from_fn(|_| {
                std::array::from_fn(|_| std::array::from_fn(|_| Register::default()))
            }),
            addr8: 0,
            bank: 0,
            port: Port::default(),
        }
    }

    pub fn set_address(&mut self, addr8: u8) {
        self.addr8 = addr8;
    }

    pub fn select_bank(&mut self, bank: u8) {
        self.bank = bank & 0b0111;
    }

    pub fn read(&self) -> u8 {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[char]
    }

    pub fn write(&mut self, value: u8) {
        let (chip, reg, char) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[char] = value & 0xF;
    }

    pub fn read_status(&self, idx: usize) -> u8 {
        let (chip, reg, _) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].status_characters[idx]
    }

    pub fn write_status(&mut self, idx: usize, value: u8) {
        let (chip, reg, _) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].status_characters[idx] = value & 0xF;
    }

    pub fn write_port(&mut self, value: u8) {
        self.port.write4(value);
    }

    pub fn attach_port(&mut self, dev: impl crate::dev::IoDevice + 'static) {
        self.port.attach(Box::new(dev));
    }

    fn decode_addr8(&self) -> (usize, usize, usize) {
        let chip = ((self.addr8 >> 6) & 0x3) as usize;
        let reg = ((self.addr8 >> 4) & 0x3) as usize;
        let char = (self.addr8 & 0xF) as usize;
        (chip, reg, char)
    }
}

impl Default for DataRam4002 {
    fn default() -> Self {
        Self::new()
    }
}
