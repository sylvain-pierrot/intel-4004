use crate::isa::Instruction;
use std::fmt;

pub struct Line {
    pub addr: u16,
    pub raw: Vec<u8>,
    pub instr: Instruction,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03X}H  ", self.addr)?;
        match self.raw.as_slice() {
            [b0] => write!(f, "{:02X}       ", b0)?,
            [b0, b1] => write!(f, "{:02X} {:02X}    ", b0, b1)?,
            _ => write!(f, "         ")?,
        }
        write!(f, "{}", self.instr)
    }
}

pub fn disassemble(bytes: &[u8]) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let addr = i as u16;
        let b0 = bytes[i];
        let b1 = bytes.get(i + 1).copied().unwrap_or(0);
        let instr = Instruction::decode(b0, b1);
        let size = instr.size();
        let raw = bytes[i..(i + size).min(bytes.len())].to_vec();
        lines.push(Line { addr, raw, instr });
        i += size;
    }
    lines
}
