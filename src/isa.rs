#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Jcn { cond: u8, addr8: u8 },
    Fim { pair: usize, imm8: u8 },
    Src { pair: usize },
    Fin { pair: usize },
    Jin { pair: usize },
    Jun { addr12: u16 },
    Jms { addr12: u16 },
    Inc { reg: usize },
    Isz { reg: usize, addr8: u8 },
    Add { reg: usize },
    Sub { reg: usize },
    Ld { reg: usize },
    Xch { reg: usize },
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
    pub fn decode(byte: u8, next_byte: Option<u8>) -> Self {
        let opr = byte >> 4;
        let opa = byte & 0xF;

        match opr {
            0x0 => Instruction::Nop,
            0x1 => Instruction::Jcn {
                cond: opa,
                addr8: next_byte.unwrap(),
            },
            0x2 => {
                let lsb = opa & 0x1;
                let pair = (opa >> 1) as usize;
                if lsb == 0 {
                    Instruction::Fim {
                        pair,
                        imm8: next_byte.unwrap(),
                    }
                } else {
                    Instruction::Src { pair }
                }
            }
            0x3 => {
                let lsb = opa & 0x1;
                let pair = (opa >> 1) as usize;
                if lsb == 0 {
                    Instruction::Fin { pair }
                } else {
                    Instruction::Jin { pair }
                }
            }
            0x4 => {
                let addr12 = ((opa as u16) << 8) | (next_byte.unwrap() as u16);
                Instruction::Jun { addr12 }
            }
            0x5 => {
                let addr12 = ((opa as u16) << 8) | (next_byte.unwrap() as u16);
                Instruction::Jms { addr12 }
            }
            0x6 => Instruction::Inc { reg: opa as usize },
            0x7 => Instruction::Isz {
                reg: opa as usize,
                addr8: next_byte.unwrap(),
            },
            0x8 => Instruction::Add { reg: opa as usize },
            0x9 => Instruction::Sub { reg: opa as usize },
            0xA => Instruction::Ld { reg: opa as usize },
            0xB => Instruction::Xch { reg: opa as usize },
            0xC => Instruction::Bbl { imm4: opa },
            0xD => Instruction::Ldm { imm4: opa },
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
            _ => Instruction::Unknown(opr),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Instruction::Jcn { .. } => 2,
            Instruction::Fim { .. } => 2,
            Instruction::Jun { .. } => 2,
            Instruction::Jms { .. } => 2,
            Instruction::Isz { .. } => 2,
            _ => 1,
        }
    }
}
