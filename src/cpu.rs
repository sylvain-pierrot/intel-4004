use crate::isa::Instruction;
// use crate::mem::Memory;

struct Cpu {
    pub acc: u8,         // 4-bit accumulator
    pub cy: bool,        // 1-bit carry flag
    pub r: [u8; 16],     // 4-bit registers (R0–R15)
    pub pc: u16,         // 12-bit program counter
    pub stack: [u16; 3], // 12-bit stack
    pub sp: usize,       // 2-bit stack pointer
    pub data_ram: [[[u8; 16]; 4]; 4],
    ram_addr: u8,
    // TODO: Memory
    // pub rom: [u8; 4096], // 8-bit words, 4096 words
    // pub ram: [u8; 4096], // 8-bit words, 4096 words

    // TODO: IO
    // pub io: u8, // 4-bit I/O
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            acc: 0,
            cy: false,
            r: [0; 16],
            pc: 0,
            stack: [0; 3],
            sp: 0,
            data_ram: [[[0; 16]; 4]; 4],
            ram_addr: 0,
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.cy = false;
        self.r = [0; 16];
        self.pc = 0;
        self.stack = [0; 3];
        self.sp = 0;
        self.data_ram = [[[0; 16]; 4]; 4];
        self.ram_addr = 0;
    }

    pub fn step(&mut self) {
        // TODO: implement a memory
        let mem = [0; 4096];

        let opcode = mem[self.pc as usize];
        let next_byte = mem[self.pc.wrapping_add(1) as usize];

        let instr = Instruction::decode(opcode, Some(next_byte));
        self.pc = self.pc.wrapping_add(instr.size() as u16);

        self.execute(instr);
    }

    pub fn execute(&mut self, instr: Instruction) {
        let pc_at_fetch = self.pc_at_fetch(&instr);

        match instr {
            Instruction::Nop => {}
            Instruction::Jcn { cond, addr8 } => {
                let invert = (cond & 0b1000) != 0;
                let test_acc = (cond & 0b0100) != 0;
                let test_cy = (cond & 0b0010) != 0;
                let _test_sig = (cond & 0b0001) != 0;

                // TODO: add TEST pin (hardware pin)
                let jump = ((test_acc && (self.acc & 0xF) == 0) || (test_cy && self.cy)) ^ invert;

                if jump {
                    self.pc = (self.pc & 0x0F00) | addr8 as u16;
                }
            }
            Instruction::Fim { pair, imm8 } => {
                let (ra, rb) = Cpu::get_pair(pair);

                self.r[ra] = (imm8 >> 4) & 0xF;
                self.r[rb] = imm8 & 0xF;
            }
            Instruction::Src { pair } => {
                let (ra, rb) = Cpu::get_pair(pair);
                self.ram_addr = ((self.r[ra] & 0xF) << 4) | (self.r[rb] & 0xF);
            }
            Instruction::Fin { pair } => {
                // TODO: implement a memory
                let mem = [0; 4096];
                let (ra, rb) = Cpu::get_pair(pair);

                let mut page = (pc_at_fetch >> 8) & 0xF;

                // Exception
                if (pc_at_fetch & 0xFF) == 0xFF {
                    page = (page + 1) & 0xF;
                }

                let addr8 = ((self.r[0] & 0xF) << 4) | (self.r[1] & 0xF);
                let addr12 = ((page as u16) << 8) | addr8 as u16;
                let data = mem[addr12 as usize];

                self.r[ra] = (data >> 4) & 0xF;
                self.r[rb] = data & 0xF;
            }
            Instruction::Jin { pair } => {
                let mut page = (pc_at_fetch >> 8) & 0xF;

                // Exception
                if (pc_at_fetch & 0xFF) == 0xFF {
                    page = (page + 1) & 0xF;
                }

                let addr8 = self.get_pair_content(pair);
                self.pc = ((page as u16) << 8) | addr8 as u16;
            }
            Instruction::Jun { addr12 } => self.pc = addr12,
            Instruction::Jms { addr12 } => {
                self.stack_write(self.pc);
                self.pc = addr12;
            }
            Instruction::Inc { reg } => self.r[reg] = (self.r[reg] + 1) & 0xF,
            Instruction::Isz { reg, addr8 } => {
                self.r[reg] = (self.r[reg] + 1) & 0xF;

                if self.r[reg] != 0 {
                    let mut page = (pc_at_fetch >> 8) & 0xF;

                    // Exception
                    if (pc_at_fetch & 0xFF) >= 0xFE {
                        page = (page + 1) & 0xF;
                    }

                    self.pc = ((page as u16) << 8) | addr8 as u16;
                }
            }
            Instruction::Add { reg } => {
                let sum = (self.acc & 0xF) + (self.r[reg] & 0xF) + (self.cy as u8);
                self.cy = sum > 0xF;
                self.acc = sum & 0xF;
            }
            Instruction::Sub { reg } => {
                let r = self.r[reg] & 0xF;
                let sum = (self.acc & 0xF) + ((!r) & 0xF) + (self.cy as u8);
                self.cy = sum > 0xF;
                self.acc = sum & 0xF;
            }
            _ => {}
        }
    }

    fn pc_at_fetch(&self, instr: &Instruction) -> u16 {
        self.pc.wrapping_sub(instr.size() as u16)
    }

    fn stack_write(&mut self, addr12: u16) {
        self.stack[self.sp] = addr12;
        self.sp = (self.sp + 1) % 3;
    }

    fn stack_read(&mut self) -> u16 {
        self.sp = (self.sp + 2) % 3;
        self.stack[self.sp]
    }

    fn get_pair(pair: usize) -> (usize, usize) {
        let ra = pair << 1;
        let rb = ra + 1;

        (ra, rb)
    }

    fn get_pair_content(&self, pair: usize) -> u8 {
        let (ra, rb) = Self::get_pair(pair);
        ((self.r[ra] & 0xF) << 4) | (self.r[rb] & 0xF)
    }
}
