#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Jcn { cond: u8, addr8: u8 },
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
    Unknown,
}

impl Instruction {
    pub fn decode(byte: u8, next_byte: u8) -> Self {
        let opr = byte >> 4;
        let opa = byte & 0xF;

        match opr {
            0x0 => Instruction::Nop,
            0x1 => Instruction::Jcn {
                cond: opa,
                addr8: next_byte,
            },
            0x2 => {
                let pair = opa >> 1;
                if opa & 0x1 == 0 {
                    Instruction::Fim {
                        pair,
                        imm8: next_byte,
                    }
                } else {
                    Instruction::Src { pair }
                }
            }
            0x3 => {
                let pair = opa >> 1;
                if opa & 0x1 == 0 {
                    Instruction::Fin { pair }
                } else {
                    Instruction::Jin { pair }
                }
            }
            0x4 => Instruction::Jun {
                addr12: ((opa as u16) << 8) | next_byte as u16,
            },
            0x5 => Instruction::Jms {
                addr12: ((opa as u16) << 8) | next_byte as u16,
            },
            0x6 => Instruction::Inc { reg: opa },
            0x7 => Instruction::Isz {
                reg: opa,
                addr8: next_byte,
            },
            0x8 => Instruction::Add { reg: opa },
            0x9 => Instruction::Sub { reg: opa },
            0xA => Instruction::Ld { reg: opa },
            0xB => Instruction::Xch { reg: opa },
            0xC => Instruction::Bbl { imm4: opa },
            0xD => Instruction::Ldm { imm4: opa },
            0xE => match opa {
                0x0 => Instruction::Wrm,
                0x1 => Instruction::Wmp,
                0x2 => Instruction::Wrr,
                0x3 => Instruction::Wpm,
                0x4 => Instruction::Wr0,
                0x5 => Instruction::Wr1,
                0x6 => Instruction::Wr2,
                0x7 => Instruction::Wr3,
                0x8 => Instruction::Sbm,
                0x9 => Instruction::Rdm,
                0xA => Instruction::Rdr,
                0xB => Instruction::Adm,
                0xC => Instruction::Rd0,
                0xD => Instruction::Rd1,
                0xE => Instruction::Rd2,
                0xF => Instruction::Rd3,
                _ => unreachable!(),
            },
            0xF => match opa {
                0x0 => Instruction::Clb,
                0x1 => Instruction::Clc,
                0x2 => Instruction::Iac,
                0x3 => Instruction::Cmc,
                0x4 => Instruction::Cma,
                0x5 => Instruction::Ral,
                0x6 => Instruction::Rar,
                0x7 => Instruction::Tcc,
                0x8 => Instruction::Dac,
                0x9 => Instruction::Tcs,
                0xA => Instruction::Stc,
                0xB => Instruction::Daa,
                0xC => Instruction::Kbp,
                0xD => Instruction::Dcl,
                _ => Instruction::Unknown,
            },
            _ => unreachable!(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Instruction::Jcn { .. }
            | Instruction::Fim { .. }
            | Instruction::Jun { .. }
            | Instruction::Jms { .. }
            | Instruction::Isz { .. } => 2,
            _ => 1,
        }
    }
}
