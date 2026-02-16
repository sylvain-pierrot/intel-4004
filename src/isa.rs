#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Jcn { cond4: u8, addr8: u8 },
    Fim { pair: u8, imm8: u8 },
    Src { pair: u8 },
    Fin { pair: u8 },
    Jin { pair: u8 },
    Jun { addr12: u16 },
    Jms { addr12: u16 },
    Inc { reg: u8 },
    Isz { reg: u8, addr8: u8 },
    Add { reg: u8 },
    Sub { reg: u8 },
    Ld { reg: u8 },
    Xch { reg: u8 },
    Bbl { imm4: u8 },
    Ldm { imm4: u8 },
    Wrm,
    Wmp,
    Wrr,
    Wpm,
    Wr0,
    Wr1,
    Wr2,
    Wr3,
    Sbm,
    Rdm,
    Rdr,
    Adm,
    Rd0,
    Rd1,
    Rd2,
    Rd3,
    Clb,
    Clc,
    Iac,
    Cmc,
    Cma,
    Ral,
    Rar,
    Tcc,
    Dac,
    Tcs,
    Stc,
    Daa,
    Kbp,
    Dcl,
    Unknown(u8),
}

impl Instruction {
    pub fn decode(opcode: u8, next_byte: Option<u8>) -> Self {   
        let hi = opcode >> 4;
        let lo = opcode & 0xF;

        match hi {
            0x0 => Instruction::Nop,
            0x1 => Instruction::Jcn { cond: lo, addr8: next_byte.unwrap() },
            0x2 => {
                let lsb = lo & 0x1;
                let pair = lo >> 1;
                if lsb == 0 {
                    Instruction::Fim { pair, imm8: next_byte.unwrap() }
                } else {
                    Instruction::Src { pair }
                }
            }
            0x3 => {
                let lsb = lo & 0x1;
                let pair = lo >> 1;
                if lsb == 0 {
                    Instruction::Fin { pair }
                } else {
                    Instruction::Jin { pair }
                }
            },
            0x4 => {
                let addr12 = (lo as u16) << 8 | next_byte.unwrap();
                Instruction::Jun { addr12 }
            },
            0x5 => {
                let addr12 = (lo as u16) << 8 | next_byte.unwrap();
                Instruction::Jms { addr12 }
            },
            0x6 => Instruction::Inc { reg: lo },
            0x7 => Instruction::Isz { reg: lo, addr8: next_byte.unwrap() },
            0x8 => Instruction::Add { reg: lo },
            0x9 => Instruction::Sub { reg: lo },
            0xA => Instruction::Ld { reg: lo },
            0xB => Instruction::Xch { reg: lo },
            0xC => Instruction::Bbl { imm4: lo },
            0xD => Instruction::Ldm { imm4: lo },
            0xE0 => Instruction::Wrm,
            0xE1 => Instruction::Wmp,
            0xE2 => Instruction::Wrr,
            0xE3 => Instruction::Wpm,
            0xE4 => Instruction::Wr0,
            0xE5 => Instruction::Wr1,
            0xE6 => Instruction::Wr2,
            0xE7 => Instruction::Wr3,
            0xE8 => Instruction::Sbm,
            0xE9 => Instruction::Rdm,
            0xEA => Instruction::Rdr,
            0xEB => Instruction::Adm,
            0xEC => Instruction::Rd0,
            0xED => Instruction::Rd1,
            0xEE => Instruction::Rd2,
            0xEF => Instruction::Rd3,
            0xF0 => Instruction::Clb,
            0xF1 => Instruction::Clc,
            0xF2 => Instruction::Iac,
            0xF3 => Instruction::Cmc,
            0xF4 => Instruction::Cma,
            0xF5 => Instruction::Ral,
            0xF6 => Instruction::Rar,
            0xF7 => Instruction::Tcc,
            0xF8 => Instruction::Dac,
            0xF9 => Instruction::Tcs,
            0xFA => Instruction::Stc,
            0xFB => Instruction::Daa,
            0xFC => Instruction::Kbp,
            0xFD => Instruction::Dcl,
            _ => Instruction::Unknown(opcode),
        }
    }

    pub fn size(&self) -> u8 {
        match self {
            Instruction::Jcn(_) => 2,
            Instruction::Fim(_) => 2,
            Instruction::Jun(_) => 2,
            Instruction::Jms(_) => 2,
            Instruction::Isz(_) => 2,
            _ => 1,
        }
    }
}