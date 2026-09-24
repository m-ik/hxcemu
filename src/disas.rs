use std::io::{self, Error, ErrorKind};

const OPCODE_BIT: u16 = 0x8000;
const PREDEFINED_REGISTERS_NUM: u16 = 16;
const SCREEN_ADDR: u16 = 16384;
const KBD_ADDR: u16 = 24576;
const CMP_SHIFT: u16 = 6;
const CMP_MASK: u16 = 0x7f;
const DST_SHIFT: u16 = 3;
const DST_MASK: u16 = 0x07;
const JMP_MASK: u16 = 0x07;

pub fn disassemble(instr: u16) -> io::Result<String> {
    if (instr & OPCODE_BIT) == 0 {
        let res = match instr {
            SCREEN_ADDR => "@SCREEN".to_string(),
            KBD_ADDR => "@KBD".to_string(),
            v if v < PREDEFINED_REGISTERS_NUM => format!("@R{v}"),
            v => format!("@{v}"),
        };
        return Ok(res);
    }

    let cmp = (instr >> CMP_SHIFT) & CMP_MASK;
    let cmp = match cmp {
        0b0_101010 => "0",
        0b0_111111 => "1",
        0b0_111010 => "-1",
        0b0_001100 => "D",
        0b0_110000 => "A",
        0b0_001101 => "!D",
        0b0_110001 => "!A",
        0b0_001111 => "-D",
        0b0_110011 => "-A",
        0b0_011111 => "D+1",
        0b0_110111 => "A+1",
        0b0_001110 => "D-1",
        0b0_110010 => "A-1",
        0b0_000010 => "D+A",
        0b0_010011 => "D-A",
        0b0_000111 => "A-D",
        0b0_000000 => "D&A",
        0b0_010101 => "D|A",
        0b1_110000 => "M",
        0b1_110001 => "!M",
        0b1_110011 => "-M",
        0b1_110111 => "M+1",
        0b1_110010 => "M-1",
        0b1_000010 => "D+M",
        0b1_010011 => "D-M",
        0b1_000111 => "M-D",
        0b1_000000 => "D&M",
        0b1_010101 => "D|M",
        _ => return Err(Error::new(ErrorKind::InvalidData, "invalid C instruction")),
    };

    let dst = (instr >> DST_SHIFT) & DST_MASK;
    let dst = match dst {
        0b000 => None,
        0b001 => Some("M"),
        0b010 => Some("D"),
        0b011 => Some("DM"),
        0b100 => Some("A"),
        0b101 => Some("AM"),
        0b110 => Some("AD"),
        0b111 => Some("ADM"),
        _ => unreachable!(),
    };

    let jmp = instr & JMP_MASK;
    let jmp = match jmp {
        0b000 => None,
        0b001 => Some("JGT"),
        0b010 => Some("JEQ"),
        0b011 => Some("JGE"),
        0b100 => Some("JLT"),
        0b101 => Some("JNE"),
        0b110 => Some("JLE"),
        0b111 => Some("JMP"),
        _ => unreachable!(),
    };

    let mut res = String::new();
    if let Some(d) = dst {
        res.push_str(d);
        res.push('=');
    }
    res.push_str(cmp);
    if let Some(j) = jmp {
        res.push(';');
        res.push_str(j);
    }

    Ok(res)
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
