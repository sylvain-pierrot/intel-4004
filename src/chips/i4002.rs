use crate::chips::Port;

#[derive(Default)]
pub struct Register {
    characters: [u8; 16],
    status_characters: [u8; 4],
}

#[derive(Default)]
pub struct DataRam4002 {
    banks: [[[Register; 4]; 4]; 8], // bank → chip → register
    addr8: u8,                      // latch d'adresse (SRC)
    bank: u8,                       // bank sélectionnée (DCL)
    port: Port,
}

impl DataRam4002 {
    pub fn set_address(&mut self, addr8: u8) {
        self.addr8 = addr8;
    }

    pub fn select_bank(&mut self, bank: u8) {
        self.bank = bank & 0b0111;
    }

    pub fn read(&self) -> u8 {
        let (chip, reg, ch) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[ch]
    }

    pub fn write(&mut self, value: u8) {
        let (chip, reg, ch) = self.decode_addr8();
        self.banks[self.bank as usize][chip][reg].characters[ch] = value & 0xF;
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
        let ch = (self.addr8 & 0xF) as usize;
        (chip, reg, ch)
    }
}
