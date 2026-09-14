use crate::alu::Alu;

const OPCODE_BIT: u16 = 0x8000;
const A_BIT: u16 = 0x1000;
const ZX_BIT: u16 = 0x0800;
const NX_BIT: u16 = 0x0400;
const ZY_BIT: u16 = 0x0200;
const NY_BIT: u16 = 0x0100;
const F_BIT: u16 = 0x0080;
const NO_BIT: u16 = 0x0040;
const DST_SHIFT: u16 = 3;
const DST_MASK: u16 = 0x07;
const DST_M: u16 = 0x01;
const DST_D: u16 = 0x02;
const DST_A: u16 = 0x04;
const JMP_MASK: u16 = 0x07;

pub struct Cpu {
    a_reg: u16,
    d_reg: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a_reg: 0,
            d_reg: 0,
            pc: 0,
        }
    }

    pub fn a_reg(&self) -> u16 {
        self.a_reg
    }

    pub fn d_reg(&self) -> u16 {
        self.d_reg
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn tick(&mut self, instr: u16, in_m: u16, reset: bool) -> (u16, bool, u16) {
        let mut out_m = 0u16;
        let mut write_m = false;
        let mut addr_m = 0u16;

        if reset {
            self.pc = 0;
            return (out_m, write_m, addr_m);
        }

        if (instr & OPCODE_BIT) == 0 {
            self.a_reg = instr;
            self.pc += 1;
            return (out_m, write_m, addr_m);
        }

        let x = self.d_reg;
        let y = if (instr & A_BIT) == 0 {
            self.a_reg
        } else {
            in_m
        };

        let zx = (instr & ZX_BIT) != 0;
        let nx = (instr & NX_BIT) != 0;
        let zy = (instr & ZY_BIT) != 0;
        let ny = (instr & NY_BIT) != 0;
        let f = (instr & F_BIT) != 0;
        let no = (instr & NO_BIT) != 0;
        let (out, zr, ng) = Alu::compute(x, y, zx, nx, zy, ny, f, no);

        out_m = out;

        let dst = (instr >> DST_SHIFT) & DST_MASK;

        if (dst & DST_M) != 0 {
            write_m = true;
            addr_m = self.a_reg;
        }
        if (dst & DST_D) != 0 {
            self.d_reg = out;
        }
        if (dst & DST_A) != 0 {
            self.a_reg = out;
        }

        let jmp = instr & JMP_MASK;

        let do_jmp = match jmp {
            1 => !ng && !zr,
            2 => zr,
            3 => !ng,
            4 => ng,
            5 => !zr,
            6 => ng || zr,
            7 => true,
            _ => false,
        };

        self.pc = if do_jmp { self.a_reg } else { self.pc + 1 };

        return (out_m, write_m, addr_m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_instructions {
        ($($name:ident: {
            state: { a: $a_reg:expr, d: $d_reg:expr, pc: $pc:expr },
            input: { instr: $instr:expr, in_m: $in_m:expr, reset: $reset:expr },
            expect: { a: $exp_a_reg:expr, d: $exp_d_reg:expr, pc: $exp_pc:expr, out_m: $out_m:expr, write_m: $write_m:expr, addr_m: $addr_m:expr }
        },)+) => {
            $(
                #[test]
                fn $name() {
                    let mut cpu = Cpu::new();
                    cpu.d_reg = $d_reg;
                    cpu.a_reg = $a_reg;
                    let (out_m, write_m, addr_m) = cpu.tick($instr, $in_m, $reset);
                    assert_eq!(cpu.pc(), $exp_pc, "pc");
                    assert_eq!(cpu.a_reg(), $exp_a_reg, "a register");
                    assert_eq!(cpu.d_reg(), $exp_d_reg, "d register");
                    assert_eq!(out_m, $out_m, "out_m");
                    assert_eq!(write_m, $write_m, "write_m");
                    assert_eq!(addr_m, $addr_m, "addr_m");
                }
            )+
        };
    }

    test_instructions!(
        ainstr: {
            state: { a: 0, d: 0, pc: 0 },
            input: { instr: 0b0_001001100010010, in_m: 0, reset: false },
            expect: { a: 0x1312, d: 0, pc: 1, out_m: 0, write_m: false, addr_m: 0}
        },
        // DM=D+M;JGT
        cinstr_d_plus_m_jgt_branch: {
            state: { a: 0x42, d: 0x1300, pc: 0 },
            input: { instr: 0b111_1_000010_011_001, in_m: 0x12, reset: false },
            expect: { a: 0x42, d: 0x1312, pc: 0x42, out_m: 0x1312, write_m: true, addr_m: 0x42 }
        },
        // M=D-A;JGE 
        cinstr_d_minus_a_jge_no_branch: {
            state: { a: 0x13, d: 0x12, pc: 0 },
            input: { instr: 0b111_0_010011_001_011, in_m: 0, reset: false },
            expect: { a: 0x13, d: 0x12, pc: 1, out_m: 0xffff, write_m: true, addr_m: 0x13 }
        },
        // AMD=1
        cinstr_set_all_dsts: {
            state: { a: 0x42, d: 0, pc: 0 },
            input: { instr: 0b111_0_111111_111_000, in_m: 0, reset: false },
            expect: { a: 1, d: 1, pc: 1, out_m: 1, write_m: true, addr_m: 0x42 }
        },
        reset: {
            state: { a: 0x52, d: 0x58, pc: 0x3a },
            input: { instr: 0, in_m: 0, reset: true },
            expect: { a: 0x52, d: 0x58, pc: 0, out_m: 0, write_m: false, addr_m: 0}
        },
    );
}
