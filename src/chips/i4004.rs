use crate::bus::Bus;
use crate::isa::Instruction;

#[derive(Default)]
pub struct Cpu4004 {
    acc: u8,         // 4-bit accumulator
    cy: u8,          // 1-bit carry flag
    r: [u8; 16],     // 4-bit registers (R0–R15)
    pc: u16,         // 12-bit program counter
    stack: [u16; 3], // 12-bit stack (3-level hardware limit)
    sp: usize,       // stack pointer (0–2)
    cycles: u64,     // elapsed clock periods (8 per 1-byte instr, 16 per 2-byte)
}

impl Cpu4004 {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn acc(&self) -> u8 {
        self.acc
    }
    pub fn cy(&self) -> u8 {
        self.cy
    }
    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn reg(&self, n: u8) -> u8 {
        self.r[(n & 0xF) as usize]
    }
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) {
        let pc0 = self.pc;
        let opcode = bus.prog_read(pc0);
        let next_byte = bus.prog_read((pc0 + 1) & 0x0FFF);

        let instr = Instruction::decode(opcode, next_byte);
        self.pc = (self.pc + instr.size() as u16) & 0x0FFF;
        self.cycles += instr.size() as u64 * 8;

        self.execute(instr, bus);

        #[cfg(feature = "debug")]
        println!(
            "PC={:03X}  {:<12}  ACC={:X} CY={} CLK={}",
            pc0, instr, self.acc, self.cy, self.cycles
        );
    }

    pub fn execute<B: Bus>(&mut self, instr: Instruction, bus: &mut B) {
        match instr {
            Instruction::Nop => {}

            Instruction::Jcn { cond, addr8 } => {
                let invert = (cond & 0b1000) != 0;
                let test_acc = (cond & 0b0100) != 0;
                let test_cy = (cond & 0b0010) != 0;
                // cond bit 0 = TEST pin (hardware), not emulated

                let jump = ((test_acc && self.acc == 0) || (test_cy && self.cy != 0)) ^ invert;
                if jump {
                    self.pc = (self.pc & 0x0F00) | addr8 as u16;
                }
            }

            Instruction::Fim { pair, imm8 } => {
                let (ra, rb) = Self::pair_regs(pair);
                self.r[ra] = (imm8 >> 4) & 0xF;
                self.r[rb] = imm8 & 0xF;
            }

            Instruction::Src { pair } => bus.data_set_address(self.pair_content(pair)),

            Instruction::Fin { pair } => {
                let (ra, rb) = Self::pair_regs(pair);
                let pc_fetch = self.pc_at_fetch(&instr);
                let page = Self::page_crossing(pc_fetch, 0xFF);
                let value = bus.prog_read((page << 8) | self.pair_content(0) as u16);
                self.r[ra] = (value >> 4) & 0xF;
                self.r[rb] = value & 0xF;
            }

            Instruction::Jin { pair } => {
                let pc_fetch = self.pc_at_fetch(&instr);
                let page = Self::page_crossing(pc_fetch, 0xFF);
                self.pc = (page << 8) | self.pair_content(pair) as u16;
            }

            Instruction::Jun { addr12 } => self.pc = addr12,

            Instruction::Jms { addr12 } => {
                self.stack_push(self.pc);
                self.pc = addr12;
            }

            Instruction::Inc { reg } => {
                self.r[reg as usize] = (self.r[reg as usize] + 1) & 0xF;
            }

            Instruction::Isz { reg, addr8 } => {
                self.r[reg as usize] = (self.r[reg as usize] + 1) & 0xF;
                if self.r[reg as usize] != 0 {
                    let pc_fetch = self.pc_at_fetch(&instr);
                    let page = Self::page_crossing(pc_fetch, 0xFE);
                    self.pc = (page << 8) | addr8 as u16;
                }
            }

            Instruction::Add { reg } => {
                let sum = self.acc + self.r[reg as usize] + self.cy;
                self.cy = (sum > 0xF) as u8;
                self.acc = sum & 0xF;
            }

            Instruction::Sub { reg } => {
                let r = self.r[reg as usize];
                let sum = self.acc + ((!r) & 0xF) + self.cy;
                self.cy = (sum > 0xF) as u8;
                self.acc = sum & 0xF;
            }

            Instruction::Ld { reg } => self.acc = self.r[reg as usize],

            Instruction::Xch { reg } => std::mem::swap(&mut self.acc, &mut self.r[reg as usize]),

            Instruction::Bbl { imm4 } => {
                self.pc = self.stack_pop();
                self.acc = imm4;
            }

            Instruction::Ldm { imm4 } => self.acc = imm4,

            Instruction::Wrm => bus.data_write(self.acc),
            Instruction::Wmp => bus.ram_port_write(self.acc),
            Instruction::Wrr => bus.rom_port_write(self.acc),
            Instruction::Wpm => {} // Write Program Memory — not supported in ROM-only config

            Instruction::Wr0 => bus.data_write_status(0, self.acc),
            Instruction::Wr1 => bus.data_write_status(1, self.acc),
            Instruction::Wr2 => bus.data_write_status(2, self.acc),
            Instruction::Wr3 => bus.data_write_status(3, self.acc),

            Instruction::Sbm => {
                let m = bus.data_read();
                let sum = self.acc + ((!m) & 0xF) + self.cy;
                self.cy = (sum > 0xF) as u8;
                self.acc = sum & 0xF;
            }
            Instruction::Rdm => self.acc = bus.data_read(),
            Instruction::Rdr => self.acc = bus.rom_port_read(),
            Instruction::Adm => {
                let m = bus.data_read();
                let sum = self.acc + m + self.cy;
                self.cy = (sum > 0xF) as u8;
                self.acc = sum & 0xF;
            }

            Instruction::Rd0 => self.acc = bus.data_read_status(0),
            Instruction::Rd1 => self.acc = bus.data_read_status(1),
            Instruction::Rd2 => self.acc = bus.data_read_status(2),
            Instruction::Rd3 => self.acc = bus.data_read_status(3),

            Instruction::Clb => {
                self.acc = 0;
                self.cy = 0;
            }
            Instruction::Clc => self.cy = 0,
            Instruction::Iac => {
                let inc = self.acc + 1;
                self.cy = (inc > 0xF) as u8;
                self.acc = inc & 0xF;
            }
            Instruction::Cmc => self.cy ^= 1,
            Instruction::Cma => self.acc = (!self.acc) & 0xF,
            Instruction::Ral => {
                let out = self.acc >> 3;
                self.acc = ((self.acc << 1) & 0xE) | self.cy;
                self.cy = out;
            }
            Instruction::Rar => {
                let out = self.acc & 1;
                self.acc = ((self.acc >> 1) & 0x7) | (self.cy << 3);
                self.cy = out;
            }
            Instruction::Tcc => {
                self.acc = self.cy;
                self.cy = 0;
            }
            Instruction::Dac => {
                self.cy = (self.acc != 0) as u8;
                self.acc = self.acc.wrapping_sub(1) & 0xF;
            }
            Instruction::Tcs => {
                self.acc = 9 + self.cy;
                self.cy = 0;
            }
            Instruction::Stc => self.cy = 1,
            Instruction::Daa => {
                if self.cy != 0 || self.acc > 9 {
                    let inc = self.acc + 6;
                    self.cy = self.cy | (inc > 0xF) as u8;
                    self.acc = inc & 0xF;
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
                };
            }
            Instruction::Dcl => bus.data_select_bank(self.acc & 0b0111),

            Instruction::Unknown => {}
        }
    }

    fn pc_at_fetch(&self, instr: &Instruction) -> u16 {
        self.pc.wrapping_sub(instr.size() as u16) & 0x0FFF
    }

    /// Returns the page of `pc`, advancing to the next page if `pc & 0xFF >= threshold`.
    fn page_crossing(pc: u16, threshold: u8) -> u16 {
        let page = (pc >> 8) & 0xF;
        if (pc & 0xFF) as u8 >= threshold {
            (page + 1) & 0xF
        } else {
            page
        }
    }

    fn stack_push(&mut self, addr12: u16) {
        self.stack[self.sp] = addr12;
        self.sp += 1;
        if self.sp == 3 {
            self.sp = 0;
        }
    }

    fn stack_pop(&mut self) -> u16 {
        if self.sp == 0 {
            self.sp = 2;
        } else {
            self.sp -= 1;
        }
        self.stack[self.sp]
    }

    fn pair_regs(pair: u8) -> (usize, usize) {
        let ra = (pair as usize) << 1;
        (ra, ra + 1)
    }

    fn pair_content(&self, pair: u8) -> u8 {
        let (ra, rb) = Self::pair_regs(pair);
        (self.r[ra] << 4) | self.r[rb]
    }
}
