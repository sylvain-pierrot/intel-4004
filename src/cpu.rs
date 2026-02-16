use crate::isa::Instruction;
// use crate::mem::Memory;

struct Cpu {
    pub acc: u8,         // 4-bit accumulator
    pub cy: bool,        // 1-bit carry flag
    pub r: [u8; 16],     // 4-bit registers (R0–R15)
    pub pc: u16,         // 12-bit program counter
    pub stack: [u16; 3], // 12-bit stack
    pub sp: u8,          // 2-bit stack pointer

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
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.cy = false;
        self.r = [0; 16];
        self.pc = 0;
        self.stack = [0; 3];
        self.sp = 0;
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
                let reg_a = (pair << 1) as usize;
                let reg_b = reg_a + 1;

                self.r[reg_a] = (imm8 >> 4) & 0xF;
                self.r[reg_b] = imm8 & 0xF;
            }
            _ => {}
        }
    }
}
