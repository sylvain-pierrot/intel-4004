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

#[test]
fn disasm_one_byte_instrs() {
    use intel_4004::disasm::disassemble;
    // NOP | WRM | STC | DAA
    let bytes = &[0x00, 0xE0, 0xFA, 0xFB];
    let lines = disassemble(bytes);
    assert_eq!(lines.len(), 4);
    assert_eq!(format!("{}", lines[0]), "000H  00       NOP");
    assert_eq!(format!("{}", lines[1]), "001H  E0       WRM");
    assert_eq!(format!("{}", lines[2]), "002H  FA       STC");
    assert_eq!(format!("{}", lines[3]), "003H  FB       DAA");
}

#[test]
fn disasm_two_byte_instrs() {
    use intel_4004::disasm::disassemble;
    // FIM P0,ABH | ISZ R3,08H | JMS 005H
    let bytes = &[0x20, 0xAB, 0x73, 0x08, 0x50, 0x05];
    let lines = disassemble(bytes);
    assert_eq!(lines.len(), 3);
    assert_eq!(format!("{}", lines[0]), "000H  20 AB    FIM P0,ABH");
    assert_eq!(format!("{}", lines[1]), "002H  73 08    ISZ R3,08H");
    assert_eq!(format!("{}", lines[2]), "004H  50 05    JMS 005H");
}

// ── NOP ───────────────────────────────────────────────────────────────────────

#[test]
fn nop() {
    let m = run(&[0x00], 1);
    assert_eq!(m.cpu().acc(), 0);
    assert_eq!(m.cpu().cy(), 0);
    assert_eq!(m.cpu().pc(), 1);
    assert_eq!(m.cycles(), 8);
}

// ── DAA carry preservation ────────────────────────────────────────────────────
//
// The bug: when cy=1 coming in from ADD and acc+6 ≤ 15, old code reset cy to 0.
// Fix: self.cy = self.cy | (inc > 0xF) as u8

#[test]
fn daa_carry_preserved_no_overflow() {
    // cy=1, acc=2 (models 9+9: binary sum 18 → acc=2,cy=1)
    // DAA: 2+6=8 ≤ 15 → acc=8, cy must stay 1
    let m = run(&[0xFA, 0xD2, 0xFB], 3); // STC | LDM 2 | DAA
    assert_eq!(m.cpu().acc(), 8);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn daa_carry_preserved_with_overflow() {
    // cy=1, acc=0xD (models a sum that already exceeded 15 again with +6)
    // DAA: 0xD+6=0x13 → acc=3, cy=1 (from the +6 overflow)
    let m = run(&[0xFA, 0xDD, 0xFB], 3); // STC | LDM 0xD | DAA
    assert_eq!(m.cpu().acc(), 3);
    assert_eq!(m.cpu().cy(), 1);
}

#[test]
fn daa_carry_in_ones_digit_model() {
    // Models ones-digit BCD: 9+8=17 → acc=1,cy=1 after ADD; DAA: 1+6=7, cy stays 1
    let m = run(&[0xFA, 0xD1, 0xFB], 3); // STC | LDM 1 | DAA
    assert_eq!(m.cpu().acc(), 7);
    assert_eq!(m.cpu().cy(), 1);
}

// ── BCD two-digit addition integration ────────────────────────────────────────

/// Run the standard 2-digit BCD add sequence for operands a and b (0–99).
/// Returns (hundreds, tens, ones) from R10, R9, R7.
fn bcd_add(a: u8, b: u8) -> (u8, u8, u8) {
    let a_hi = a / 10;
    let a_lo = a % 10;
    let b_hi = b / 10;
    let b_lo = b % 10;
    #[rustfmt::skip]
    let prog: &[u8] = &[
        0x20, (a_hi << 4) | a_lo, // FIM P0, (tens_a)(ones_a)
        0x22, (b_hi << 4) | b_lo, // FIM P1, (tens_b)(ones_b)
        0xF1,                      // CLC
        // ones: LD R1; XCH R6; LD R3; ADD R6; DAA; XCH R7
        0xA1, 0xB6, 0xA3, 0x86, 0xFB, 0xB7,
        // tens: LD R0; XCH R8; LD R2; ADD R8; DAA; XCH R9
        0xA0, 0xB8, 0xA2, 0x88, 0xFB, 0xB9,
        // hundreds: TCC; XCH R10
        0xF7, 0xBA,
    ];
    let m = run(prog, 17);
    (m.cpu().reg(10), m.cpu().reg(9), m.cpu().reg(7))
}

#[test]
fn bcd_add_zero_plus_zero() {
    assert_eq!(bcd_add(0, 0), (0, 0, 0));
}

#[test]
fn bcd_add_simple() {
    assert_eq!(bcd_add(1, 1), (0, 0, 2));
    assert_eq!(bcd_add(4, 3), (0, 0, 7));
}

#[test]
fn bcd_add_ones_carry() {
    // 9+9=18: ones carry propagates to tens
    assert_eq!(bcd_add(9, 9), (0, 1, 8));
}

#[test]
fn bcd_add_result_100() {
    assert_eq!(bcd_add(50, 50), (1, 0, 0));
    assert_eq!(bcd_add(99, 1), (1, 0, 0));
}

#[test]
fn bcd_add_73_plus_58() {
    assert_eq!(bcd_add(73, 58), (1, 3, 1)); // 131
}

#[test]
fn bcd_add_99_plus_99() {
    assert_eq!(bcd_add(99, 99), (1, 9, 8)); // 198
}

// ── SUB borrow ────────────────────────────────────────────────────────────────

#[test]
fn sub_borrow() {
    // STC | LDM 5 | XCH R0 | LDM 3 | SUB R0
    // 3 + (~5 & 0xF) + 1 = 3 + 10 + 1 = 14 ≤ 15 → acc=14, cy=0 (borrow)
    let m = run(&[0xFA, 0xD5, 0xB0, 0xD3, 0x90], 5);
    assert_eq!(m.cpu().acc(), 14);
    assert_eq!(m.cpu().cy(), 0);
}

// ── JCN additional conditions ─────────────────────────────────────────────────

#[test]
fn jcn_carry_clear_not_taken() {
    // CLC; JCN 2H,06H (jump if cy≠0) → cy=0, not taken; LDM 8 at 0x003
    let m = run(&[0xF1, 0x12, 0x06, 0xD8], 3);
    assert_eq!(m.cpu().acc(), 8);
}

#[test]
fn jcn_inverted_carry_zero_taken() {
    // CLC; JCN AH,06H (cond=1010: invert|(cy≠0) → jump if cy=0) → taken; LDM 9 at 0x006
    let m = run(&[0xF1, 0x1A, 0x06, 0x00, 0x00, 0x00, 0xD9], 3);
    assert_eq!(m.cpu().acc(), 9);
}

// ── Nested JMS ────────────────────────────────────────────────────────────────

#[test]
fn jms_nested() {
    // 0x000: JMS 006H   → push 0x002, jump to 0x006
    // 0x002: BBL 2      ← outer return point (not reached first)
    // 0x003..0x005: pad
    // 0x006: JMS 00BH   → push 0x008, jump to 0x00B
    // 0x008: BBL 3      ← inner return point
    // 0x009..0x00A: pad
    // 0x00B: BBL 5      ← inner body, returns 5 to 0x008; then BBL 3 returns 3 to 0x002
    #[rustfmt::skip]
    let prog = &[
        0x50, 0x06, // 000: JMS 006H
        0xC2,       // 002: BBL 2
        0x00, 0x00, 0x00,
        0x50, 0x0B, // 006: JMS 00BH
        0xC3,       // 008: BBL 3
        0x00, 0x00,
        0xC5,       // 00B: BBL 5
    ];
    let m = run(prog, 4);
    assert_eq!(m.cpu().acc(), 3);
    assert_eq!(m.cpu().pc(), 0x002);
}

// ── ISZ counting loop ─────────────────────────────────────────────────────────

#[test]
fn isz_loop_counts_to_wrap() {
    // FIM P0, 0xE0  (R0=0xE)
    // 0x002: ISZ R0, 002H  → loop while R0≠0 (R0: 0xE→0xF→0x0)
    // 0x004: LDM 9
    // Steps: 1(FIM) + 2(ISZ×2) + 1(LDM) = 4
    let m = run(&[0x20, 0xE0, 0x70, 0x02, 0xD9], 4);
    assert_eq!(m.cpu().acc(), 9);
    assert_eq!(m.cpu().reg(0), 0);
}

// ── Memory: SRC + WRM + RDM ───────────────────────────────────────────────────

#[test]
fn wrm_rdm_roundtrip() {
    // FIM P0,0x12; SRC P0; LDM 7; WRM; LDM 0; RDM → acc=7
    let m = run(&[0x20, 0x12, 0x21, 0xD7, 0xE0, 0xD0, 0xE9], 6);
    assert_eq!(m.cpu().acc(), 7);
}

#[test]
fn adm_adds_memory() {
    // FIM P0,0x00; SRC P0; LDM 3; WRM; LDM 4; ADM → acc=4+3=7, cy=0
    let m = run(&[0x20, 0x00, 0x21, 0xD3, 0xE0, 0xD4, 0xEB], 6);
    assert_eq!(m.cpu().acc(), 7);
    assert_eq!(m.cpu().cy(), 0);
}

#[test]
fn sbm_subtracts_memory() {
    // FIM P0,0x00; SRC P0; LDM 2; WRM; STC; LDM 9; SBM
    // 9 + (~2 & 0xF) + 1 = 9 + 13 + 1 = 23 → acc=7, cy=1 (no borrow)
    let m = run(&[0x20, 0x00, 0x21, 0xD2, 0xE0, 0xFA, 0xD9, 0xE8], 7);
    assert_eq!(m.cpu().acc(), 7);
    assert_eq!(m.cpu().cy(), 1);
}

// ── Status characters: WR0–3 / RD0–3 ─────────────────────────────────────────

#[test]
fn wr0_rd0_roundtrip() {
    // FIM P0,0x10; SRC P0; LDM 0xA; WR0; LDM 0; RD0 → acc=0xA
    let m = run(&[0x20, 0x10, 0x21, 0xDA, 0xE4, 0xD0, 0xEC], 6);
    assert_eq!(m.cpu().acc(), 0xA);
}

#[test]
fn status_slots_are_independent() {
    // FIM P0,0x20; SRC P0; LDM 5; WR1; LDM 0xC; WR2; LDM 0; RD1 → acc=5 (not 0xC)
    let m = run(&[0x20, 0x20, 0x21, 0xD5, 0xE5, 0xDC, 0xE6, 0xD0, 0xED], 8);
    assert_eq!(m.cpu().acc(), 5);
}

// ── DCL bank isolation ────────────────────────────────────────────────────────

#[test]
fn dcl_banks_are_isolated() {
    // Write 3 to bank 0 addr 0x00; switch to bank 1; write 7; switch back; read → 3
    #[rustfmt::skip]
    let prog: &[u8] = &[
        0x20, 0x00, // FIM P0,0x00
        0x21,       // SRC P0       addr=0x00
        0xD3, 0xE0, // LDM 3; WRM   → bank0[0x00]=3
        0xD1, 0xFD, // LDM 1; DCL   → select bank 1
        0xD7, 0xE0, // LDM 7; WRM   → bank1[0x00]=7
        0xD0, 0xFD, // LDM 0; DCL   → select bank 0
        0xD0, 0xE9, // LDM 0; RDM   → acc = bank0[0x00] = 3
    ];
    let m = run(prog, 12);
    assert_eq!(m.cpu().acc(), 3);
}

// ── FIN (fetch indirect) ──────────────────────────────────────────────────────

#[test]
fn fin_loads_from_rom() {
    // FIM P0,0x05 → R0=0,R1=5; FIN P1 → reads ROM[0x005]=0xAB → R2=0xA, R3=0xB
    let prog: &[u8] = &[0x20, 0x05, 0x32, 0x00, 0x00, 0xAB];
    let m = run(prog, 2);
    assert_eq!(m.cpu().reg(2), 0xA);
    assert_eq!(m.cpu().reg(3), 0xB);
}

// ── JIN (jump indirect) ───────────────────────────────────────────────────────

#[test]
fn jin_jumps_to_pair() {
    // FIM P1,0x07 → R2=0,R3=7; NOP; JIN P1 → jump to 0x007; LDM 0xB
    let prog: &[u8] = &[0x22, 0x07, 0x00, 0x33, 0x00, 0x00, 0x00, 0xDB];
    let m = run(prog, 4);
    assert_eq!(m.cpu().acc(), 0xB);
}

// ── run_until ─────────────────────────────────────────────────────────────────

#[test]
fn run_until_stops_on_condition() {
    use intel_4004::bus::simple::SimpleBus;
    use intel_4004::chips::{DataRam4002, Rom4001};
    use intel_4004::machine::Machine;

    let prog = &[0xD5, 0x00, 0x00]; // LDM 5; NOP; NOP
    let bus = SimpleBus::new(Rom4001::from_bytes(prog), DataRam4002::default());
    let mut m = Machine::new(bus);
    m.run_until(|cpu| cpu.acc() != 0);
    assert_eq!(m.cpu().acc(), 5);
    assert_eq!(m.cpu().pc(), 1);
}
