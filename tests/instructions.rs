use intel_4004::bus::simple::SimpleBus;
use intel_4004::chips::{DataRam4002, Rom4001};
use intel_4004::machine::Machine;

fn run(bytes: &[u8], steps: usize) -> Machine<SimpleBus> {
    let bus = SimpleBus::new(Rom4001::from_bytes(bytes), DataRam4002::default());
    let mut m = Machine::new(bus);
    m.run_steps(steps);
    m
}

// ── LDM ──────────────────────────────────────────────────────────────────────

#[test]
fn ldm() {
    let m = run(&[0xD7], 1); // LDM 7
    assert_eq!(m.cpu().acc(), 7);
    assert_eq!(m.cpu().cy(), 0);
}

// ── ADD ──────────────────────────────────────────────────────────────────────

#[test]
fn add_no_carry() {
    // CLC | LDM 3 | XCH R0 | LDM 4 | ADD R0  →  4+3+0 = 7, cy=0
    let m = run(&[0xF1, 0xD3, 0xB0, 0xD4, 0x80], 5);
    assert_eq!(m.cpu().acc(), 7);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn add_carry_out() {
    // CLC | LDM 9 | XCH R0 | LDM 9 | ADD R0  →  9+9+0 = 18 → acc=2, cy=1
    let m = run(&[0xF1, 0xD9, 0xB0, 0xD9, 0x80], 5);
    assert_eq!(m.cpu().acc(), 2);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn add_carry_in() {
    // STC | LDM 1 | XCH R0 | LDM 1 | ADD R0  →  1+1+1 = 3, cy=0
    let m = run(&[0xFA, 0xD1, 0xB0, 0xD1, 0x80], 5);
    assert_eq!(m.cpu().acc(), 3);
    assert_eq!(m.cpu().cy(), 0);
}

// ── SUB ──────────────────────────────────────────────────────────────────────

#[test]
fn sub_no_borrow() {
    // STC | LDM 5 | XCH R0 | LDM 7 | SUB R0  →  7+(~5&F)+1 = 7+10+1=18 → acc=2, cy=1
    let m = run(&[0xFA, 0xD5, 0xB0, 0xD7, 0x90], 5);
    assert_eq!(m.cpu().acc(), 2);
    assert_eq!(m.cpu().cy(), 1);
}

// ── IAC / DAC ─────────────────────────────────────────────────────────────────

#[test]
fn iac_normal() {
    let m = run(&[0xD4, 0xF2], 2); // LDM 4 | IAC → acc=5
    assert_eq!(m.cpu().acc(), 5);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn iac_overflow() {
    let m = run(&[0xDF, 0xF2], 2); // LDM 15 | IAC → acc=0, cy=1
    assert_eq!(m.cpu().acc(), 0);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn dac_normal() {
    let m = run(&[0xD5, 0xF8], 2); // LDM 5 | DAC → acc=4, cy=1
    assert_eq!(m.cpu().acc(), 4);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn dac_underflow() {
    let m = run(&[0xD0, 0xF8], 2); // LDM 0 | DAC → acc=15, cy=0 (borrow)
    assert_eq!(m.cpu().acc(), 15);
    assert_eq!(m.cpu().cy(), 0);
}

// ── CMA / CMC / STC / CLC / CLB ──────────────────────────────────────────────

#[test]
fn cma() {
    let m = run(&[0xD5, 0xF4], 2); // LDM 5 (0101) | CMA → 1010 = 0xA
    assert_eq!(m.cpu().acc(), 0xA);
}

#[test]
fn cmc_toggles() {
    let m = run(&[0xF1, 0xF3], 2); // CLC | CMC → cy=1
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn stc() {
    let m = run(&[0xFA], 1);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn clc() {
    let m = run(&[0xFA, 0xF1], 2); // STC | CLC → cy=0
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn clb() {
    let m = run(&[0xFA, 0xD7, 0xF0], 3); // STC | LDM 7 | CLB → acc=0, cy=0
    assert_eq!(m.cpu().acc(), 0);
    assert_eq!(m.cpu().cy(), 0);
}

// ── RAL / RAR ─────────────────────────────────────────────────────────────────

#[test]
fn ral_no_carry_in() {
    // CLC | LDM 5 (0101) | RAL → 1010 = 0xA, cy=0
    let m = run(&[0xF1, 0xD5, 0xF5], 3);
    assert_eq!(m.cpu().acc(), 0xA);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn ral_carry_in_and_out() {
    // STC | LDM 9 (1001) | RAL → (1001<<1)|1 & 0xF = 0011 = 3, cy=1
    let m = run(&[0xFA, 0xD9, 0xF5], 3);
    assert_eq!(m.cpu().acc(), 0x3);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn rar_no_carry_in() {
    // CLC | LDM 6 (0110) | RAR → 0011 = 3, cy=0
    let m = run(&[0xF1, 0xD6, 0xF6], 3);
    assert_eq!(m.cpu().acc(), 0x3);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn rar_carry_in_and_out() {
    // STC | LDM 5 (0101) | RAR → (0101>>1)|(1<<3) = 1010 = 0xA, cy=1
    let m = run(&[0xFA, 0xD5, 0xF6], 3);
    assert_eq!(m.cpu().acc(), 0xA);
    assert_eq!(m.cpu().cy(), 1);
}

// ── TCC / TCS / DAA / KBP ────────────────────────────────────────────────────

#[test]
fn tcc() {
    let m = run(&[0xFA, 0xF7], 2); // STC | TCC → acc=1, cy=0
    assert_eq!(m.cpu().acc(), 1);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn tcs_no_carry() {
    let m = run(&[0xF1, 0xF9], 2); // CLC | TCS → acc=9, cy=0
    assert_eq!(m.cpu().acc(), 9);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn tcs_with_carry() {
    let m = run(&[0xFA, 0xF9], 2); // STC | TCS → acc=10, cy=0
    assert_eq!(m.cpu().acc(), 10);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn daa_no_adjust() {
    let m = run(&[0xD5, 0xFB], 2); // LDM 5 | DAA → no change
    assert_eq!(m.cpu().acc(), 5);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn daa_adjusts() {
    let m = run(&[0xDA, 0xFB], 2); // LDM 10 (>9) | DAA → 10+6=16 → acc=0, cy=1
    assert_eq!(m.cpu().acc(), 0);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn kbp_valid() {
    let m = run(&[0xD4, 0xFC], 2); // LDM 4 (only bit 2 set) | KBP → acc=3
    assert_eq!(m.cpu().acc(), 3);
}

#[test]
fn kbp_invalid() {
    let m = run(&[0xD3, 0xFC], 2); // LDM 3 (two bits set) | KBP → acc=0xF
    assert_eq!(m.cpu().acc(), 0xF);
}

// ── LD / XCH / INC / ISZ / FIM ───────────────────────────────────────────────

#[test]
fn ld() {
    // FIM P0, 0xAB → R0=0xA, R1=0xB; LD R1 → acc=0xB
    let m = run(&[0x20, 0xAB, 0xA1], 2);
    assert_eq!(m.cpu().acc(), 0xB);
}

#[test]
fn xch() {
    // LDM 7 | XCH R2 → acc=R2(0)=0, R2=7
    let m = run(&[0xD7, 0xB2], 2);
    assert_eq!(m.cpu().acc(), 0);
    assert_eq!(m.cpu().reg(2), 7);
}

#[test]
fn inc() {
    let m = run(&[0x63], 1); // INC R3 → R3: 0→1
    assert_eq!(m.cpu().reg(3), 1);
}

#[test]
fn inc_wraps() {
    // FIM P1, 0xFF → R2=0xF,R3=0xF; INC R3 → (0xF+1)&0xF = 0
    let m = run(&[0x22, 0xFF, 0x63], 2);
    assert_eq!(m.cpu().reg(3), 0);
}

#[test]
fn fim() {
    // FIM P2, 0xCD → R4=0xC, R5=0xD
    let m = run(&[0x24, 0xCD], 1);
    assert_eq!(m.cpu().reg(4), 0xC);
    assert_eq!(m.cpu().reg(5), 0xD);
}

#[test]
fn isz_nonzero_jumps() {
    // ISZ R0, 05H → R0: 0→1 (≠0), jump to 0x005; LDM 7 at 0x005
    let m = run(&[0x70, 0x05, 0x00, 0x00, 0x00, 0xD7], 2);
    assert_eq!(m.cpu().acc(), 7);
}

#[test]
fn isz_zero_no_jump() {
    // FIM P0, 0xFF → R1=0xF; ISZ R1, 06H → R1: 0xF→0 (=0), no jump; LDM 7 at 0x004
    let m = run(&[0x20, 0xFF, 0x71, 0x06, 0xD7, 0x00], 3);
    assert_eq!(m.cpu().acc(), 7);
}

// ── JUN / JMS+BBL ────────────────────────────────────────────────────────────

#[test]
fn jun() {
    // JUN 005H; <padding>; LDM 9 at 0x005
    let m = run(&[0x40, 0x05, 0x00, 0x00, 0x00, 0xD9], 2);
    assert_eq!(m.cpu().acc(), 9);
}

#[test]
fn jms_bbl() {
    // 0x000: JMS 005H → push 0x002, jump to 0x005
    // 0x002: LDM 1    ← not reached
    // 0x005: LDM 7    ← subroutine body
    // 0x006: BBL 3    ← return with acc=3, pc→0x002
    let m = run(&[0x50, 0x05, 0xD1, 0x00, 0x00, 0xD7, 0xC3], 3);
    assert_eq!(m.cpu().acc(), 3);
    assert_eq!(m.cpu().pc(), 0x002);
}

// ── JCN ──────────────────────────────────────────────────────────────────────

#[test]
fn jcn_acc_zero_jumps() {
    // ACC=0; JCN 4H,05H (cond=0100: jump if acc==0) → taken; LDM 7 at 0x005
    let m = run(&[0x14, 0x05, 0x00, 0x00, 0x00, 0xD7], 2);
    assert_eq!(m.cpu().acc(), 7);
}

#[test]
fn jcn_acc_nonzero_no_jump() {
    // LDM 5; JCN 4H,06H → acc≠0, not taken; LDM 7 at 0x003
    let m = run(&[0xD5, 0x14, 0x06, 0xD7], 3);
    assert_eq!(m.cpu().acc(), 7);
}

#[test]
fn jcn_carry_set_jumps() {
    // STC; JCN 2H,06H (cond=0010: jump if cy≠0) → taken; LDM 9 at 0x006
    let m = run(&[0xFA, 0x12, 0x06, 0x00, 0x00, 0x00, 0xD9], 3);
    assert_eq!(m.cpu().acc(), 9);
}

#[test]
fn jcn_inverted_acc_nonzero_jumps() {
    // LDM 5; JCN CH,07H (cond=1100: invert|acc → jump if acc≠0) → taken; LDM 9 at 0x007
    let m = run(&[0xD5, 0x1C, 0x07, 0x00, 0x00, 0x00, 0x00, 0xD9], 3);
    assert_eq!(m.cpu().acc(), 9);
}

// ── Cycle counter ─────────────────────────────────────────────────────────────

#[test]
fn cycles_one_byte() {
    // 3 × IAC (1 byte each) = 3 × 8 = 24 clock periods
    let m = run(&[0xF2, 0xF2, 0xF2], 3);
    assert_eq!(m.cycles(), 24);
}

#[test]
fn cycles_two_byte() {
    // JUN 003H (2 bytes = 16) + LDM 0 (1 byte = 8) = 24
    let m = run(&[0x40, 0x03, 0x00, 0xD0], 2);
    assert_eq!(m.cycles(), 24);
}

#[test]
fn cycles_mixed() {
    // FIM P0,0xAB (2 bytes = 16) + INC R0 (1 byte = 8) = 24
    let m = run(&[0x20, 0xAB, 0x60], 2);
    assert_eq!(m.cycles(), 24);
}

// ── Disassembler ─────────────────────────────────────────────────────────────

#[test]
fn disasm_display() {
    use intel_4004::disasm::disassemble;
    let bytes = &[0xD4, 0xE1, 0x40, 0x10];
    let lines = disassemble(bytes);
    assert_eq!(lines.len(), 3);
    assert_eq!(format!("{}", lines[0]), "000H  D4       LDM 4");
    assert_eq!(format!("{}", lines[1]), "001H  E1       WMP");
    assert_eq!(format!("{}", lines[2]), "002H  40 10    JUN 010H");
}
