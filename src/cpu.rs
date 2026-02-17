use std::mem;

use crate::isa::Instruction;
use crate::memory::data_ram::DataRam;

pub struct Cpu {
    acc: u8,         // 4-bit accumulator
    cy: bool,        // 1-bit carry flag
    r: [u8; 16],     // 4-bit registers (R0–R15)
    pc: u16,         // 12-bit program counter
    stack: [u16; 3], // 12-bit stack
    sp: usize,       // 2-bit stack pointer
    data_ram: DataRam,
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
            data_ram: DataRam::new(),
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.cy = false;
        self.r = [0; 16];
        self.pc = 0;
        self.stack = [0; 3];
        self.sp = 0;
        self.data_ram = DataRam::new();
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
            Instruction::Src { pair } => self.data_ram.src(self.get_pair_content(pair)),
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
            Instruction::Xch { reg } => mem::swap(&mut self.acc, &mut self.r[reg]),
            Instruction::Bbl { imm4 } => {
                self.pc = self.stack_read();
                self.acc = imm4;
            }
            Instruction::Ldm { imm4 } => self.acc = imm4,
            Instruction::Wrm => self.data_ram.write_main(self.acc),
            Instruction::Wmp => todo!(),
            Instruction::Wrr => todo!(),
            Instruction::Wr0 => todo!(),
            Instruction::Wr1 => todo!(),
            Instruction::Wr2 => todo!(),
            Instruction::Wr3 => todo!(),
            Instruction::Sbm => todo!(),
            Instruction::Rdm => todo!(),
            Instruction::Rdr => todo!(),
            Instruction::Adm => todo!(),
            Instruction::Rd0 => todo!(),
            Instruction::Rd1 => todo!(),
            Instruction::Rd2 => todo!(),
            Instruction::Rd3 => todo!(),
            Instruction::Clb => todo!(),
            Instruction::Clc => todo!(),
            Instruction::Iac => todo!(),
            Instruction::Cmc => todo!(),
            Instruction::Cma => todo!(),
            Instruction::Ral => todo!(),
            Instruction::Rar => todo!(),
            Instruction::Tcc => todo!(),
            Instruction::Dac => todo!(),
            Instruction::Tcs => todo!(),
            Instruction::Stc => todo!(),
            Instruction::Daa => todo!(),
            Instruction::Kbp => todo!(),
            Instruction::Dcl => self.data_ram.dcl((self.acc & 0b0111) as usize),
            _ => {}
        }
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

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
