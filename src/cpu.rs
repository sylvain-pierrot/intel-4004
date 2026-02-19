use crate::isa::Instruction;
use crate::memory::Memory;

pub struct Cpu<M: Memory + Default> {
    acc: u8,         // 4-bit accumulator
    cy: bool,        // 1-bit carry flag
    r: [u8; 16],     // 4-bit registers (R0–R15)
    pc: u16,         // 12-bit program counter
    stack: [u16; 3], // 12-bit stack
    sp: usize,       // 2-bit stack pointer
    mem: M,
}

impl<M: Memory + Default> Cpu<M> {
    pub fn new() -> Self {
        Self {
            acc: 0,
            cy: false,
            r: [0; 16],
            pc: 0,
            stack: [0; 3],
            sp: 0,
            mem: M::default(),
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.cy = false;
        self.r = [0; 16];
        self.pc = 0;
        self.stack = [0; 3];
        self.sp = 0;
        self.mem = M::default();
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
                let (ra, rb) = Cpu::<M>::get_pair(pair);

                self.r[ra] = (imm8 >> 4) & 0xF;
                self.r[rb] = imm8 & 0xF;
            }
            Instruction::Src { pair } => self.mem.src(self.get_pair_content(pair)),
            Instruction::Fin { pair } => {
                // TODO: implement a memory
                let mem = [0; 4096];
                let (ra, rb) = Cpu::<M>::get_pair(pair);

                let mut page = (pc_at_fetch >> 8) & 0xF;

                // Exception
                if (pc_at_fetch & 0xFF) == 0xFF {
                    page = (page + 1) & 0xF;
                }

                let addr8 = ((self.r[0] & 0xF) << 4) | (self.r[1] & 0xF);
                let addr12 = (page << 8) | addr8 as u16;
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
                self.pc = (page << 8) | addr8 as u16;
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

                    self.pc = (page << 8) | addr8 as u16;
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
            Instruction::Ld { reg } => self.acc = self.r[reg],
            Instruction::Xch { reg } => std::mem::swap(&mut self.acc, &mut self.r[reg]),
            Instruction::Bbl { imm4 } => {
                self.pc = self.stack_read();
                self.acc = imm4;
            }
            Instruction::Ldm { imm4 } => self.acc = imm4,
            Instruction::Wrm => self.mem.write_character(self.acc),
            Instruction::Wmp => todo!(),
            Instruction::Wrr => todo!(),
            Instruction::Wr0 => self.mem.write_status_character(0, self.acc),
            Instruction::Wr1 => self.mem.write_status_character(1, self.acc),
            Instruction::Wr2 => self.mem.write_status_character(2, self.acc),
            Instruction::Wr3 => self.mem.write_status_character(3, self.acc),
            Instruction::Sbm => todo!(),
            Instruction::Rdm => todo!(),
            Instruction::Rdr => todo!(),
            Instruction::Adm => todo!(),
            Instruction::Rd0 => self.acc = self.mem.read_status_character(0),
            Instruction::Rd1 => self.acc = self.mem.read_status_character(1),
            Instruction::Rd2 => self.acc = self.mem.read_status_character(2),
            Instruction::Rd3 => self.acc = self.mem.read_status_character(3),
            Instruction::Clb => {
                self.acc = 0;
                self.cy = false;
            }
            Instruction::Clc => self.cy = false,
            Instruction::Iac => {
                let inc = self.acc.wrapping_add(1);
                self.cy = inc > 0xF;
                self.acc = inc & 0xF;
            }
            Instruction::Cmc => self.cy = !self.cy,
            Instruction::Cma => self.acc = !self.acc,
            Instruction::Ral => {
                let hsb = (self.acc >> 3) & 0b1;
                self.acc = ((self.acc << 1) & 0b1110) | (self.cy as u8) & 0b1;
                self.cy = hsb != 0;
            }
            Instruction::Rar => {
                let lsb = self.acc & 0b1;
                self.acc = ((self.acc >> 1) & 0b0111) | ((self.cy as u8) & 0b1) << 3;
                self.cy = lsb != 0;
            }
            Instruction::Tcc => {
                self.acc = 0;
                self.acc = (self.acc & 0b1110) | (self.cy as u8) & 0b1;
                self.cy = false;
            }
            Instruction::Dac => {
                let dec = self.acc.wrapping_sub(1);
                self.cy = dec <= 0xF;
                self.acc = dec & 0xF;
            }
            Instruction::Tcs => {
                self.acc = if self.cy { 0xA } else { 0x9 };
                self.cy = false;
            }
            Instruction::Stc => self.cy = true,
            Instruction::Daa => {
                if self.cy || self.acc > 0x9 {
                    let inc = self.acc.wrapping_add(6);
                    self.cy = inc > 0xF;
                }
            }
            Instruction::Kbp => {
                self.acc = match self.acc {
                    0x0 => 0x0,
                    0x1 => 0x1,
                    0x2 => 0x2,
                    0x4 => 0x3,
                    0x8 => 0x4,
                    _ => 0xF,
                }
            }
            Instruction::Dcl => self.mem.dcl(self.acc & 0b0111),
            _ => {}
        };
    }

    fn pc_at_fetch(&mut self, instr: &Instruction) -> u16 {
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

impl<M: Memory + Default> Default for Cpu<M> {
    fn default() -> Self {
        Self::new()
    }
}
