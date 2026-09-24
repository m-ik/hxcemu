use std::io;

use crate::isa;

pub fn disassemble(instr: u16) -> io::Result<String> {
    let instr = isa::Instruction::parse(instr)?;
    Ok(instr.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disassemble_ainstr_predefined() {
        let instr = disassemble(0b0_000000000001001).unwrap();
        assert_eq!(instr, "@R9");
    }

    #[test]
    fn disassemble_ainstr_addr() {
        let instr = disassemble(0b0_000000000010011).unwrap();
        assert_eq!(instr, "@19");
    }

    #[test]
    fn disassemble_ainstr_screen() {
        let instr = disassemble(0b0_100000000000000).unwrap();
        assert_eq!(instr, "@SCREEN");
    }

    #[test]
    fn disassemble_ainstr_kbd() {
        let instr = disassemble(0b0_110000000000000).unwrap();
        assert_eq!(instr, "@KBD");
    }

    #[test]
    fn disassemble_cinstr_valid() {
        let instr = disassemble(0b111_0_000000_001_001).unwrap();
        assert_eq!(instr, "M=D&A;JGT");
    }
}
