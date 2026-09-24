use std::{
    fmt,
    io::{self, Error, ErrorKind},
};

pub const INSTRUCTION_TYPE_BIT: u16 = 0x8000;

pub const PREDEFINED_REGISTERS_NUM: u16 = 16;

pub const SCREEN_ADDR: u16 = 16384;
pub const KBD_ADDR: u16 = 24576;

pub const A_BIT: u16 = 0x1000;
pub const CMP_SHIFT: u16 = 6;
pub const CMP_MASK: u16 = 0x7f;
pub const DST_SHIFT: u16 = 3;
pub const DST_MASK: u16 = 0x07;
pub const DST_M: u16 = 0x01;
pub const DST_D: u16 = 0x02;
pub const DST_A: u16 = 0x04;
pub const JMP_MASK: u16 = 0x07;

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    NegOne,
    D,
    A,
    NotD,
    NotA,
    NegD,
    NegA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    NegM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug)]
pub enum Dest {
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[derive(Debug)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug)]
pub enum Instruction {
    A(u16),
    C {
        comp: Comp,
        dest: Option<Dest>,
        jump: Option<Jump>,
    },
}

impl Comp {
    pub fn parse(bits: u8) -> Option<Comp> {
        match bits {
            0b0_101010 => Some(Comp::Zero),
            0b0_111111 => Some(Comp::One),
            0b0_111010 => Some(Comp::NegOne),
            0b0_001100 => Some(Comp::D),
            0b0_110000 => Some(Comp::A),
            0b0_001101 => Some(Comp::NotD),
            0b0_110001 => Some(Comp::NotA),
            0b0_001111 => Some(Comp::NegD),
            0b0_110011 => Some(Comp::NegA),
            0b0_011111 => Some(Comp::DPlusOne),
            0b0_110111 => Some(Comp::APlusOne),
            0b0_001110 => Some(Comp::DMinusOne),
            0b0_110010 => Some(Comp::AMinusOne),
            0b0_000010 => Some(Comp::DPlusA),
            0b0_010011 => Some(Comp::DMinusA),
            0b0_000111 => Some(Comp::AMinusD),
            0b0_000000 => Some(Comp::DAndA),
            0b0_010101 => Some(Comp::DOrA),
            0b1_110000 => Some(Comp::M),
            0b1_110001 => Some(Comp::NotM),
            0b1_110011 => Some(Comp::NegM),
            0b1_110111 => Some(Comp::MPlusOne),
            0b1_110010 => Some(Comp::MMinusOne),
            0b1_000010 => Some(Comp::DPlusM),
            0b1_010011 => Some(Comp::DMinusM),
            0b1_000111 => Some(Comp::MMinusD),
            0b1_000000 => Some(Comp::DAndM),
            0b1_010101 => Some(Comp::DOrM),
            _ => None,
        }
    }
}

impl Dest {
    pub fn parse(bits: u8) -> Option<Dest> {
        match bits {
            0b001 => Some(Dest::M),
            0b010 => Some(Dest::D),
            0b011 => Some(Dest::DM),
            0b100 => Some(Dest::A),
            0b101 => Some(Dest::AM),
            0b110 => Some(Dest::AD),
            0b111 => Some(Dest::ADM),
            _ => None,
        }
    }
}

impl Jump {
    pub fn parse(bits: u8) -> Option<Jump> {
        match bits {
            0b001 => Some(Jump::JGT),
            0b010 => Some(Jump::JEQ),
            0b011 => Some(Jump::JGE),
            0b100 => Some(Jump::JLT),
            0b101 => Some(Jump::JNE),
            0b110 => Some(Jump::JLE),
            0b111 => Some(Jump::JMP),
            _ => None,
        }
    }
}

impl Instruction {
    pub fn parse(instr: u16) -> io::Result<Instruction> {
        if (instr & INSTRUCTION_TYPE_BIT) == 0 {
            return Ok(Instruction::A(instr));
        }

        let comp = ((instr >> CMP_SHIFT) & CMP_MASK) as u8;
        let dest = ((instr >> DST_SHIFT) & DST_MASK) as u8;
        let jump = (instr & JMP_MASK) as u8;

        let comp = Comp::parse(comp).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("invalid comp: {:07b}", comp),
            )
        })?;

        Ok(Instruction::C {
            comp: comp,
            dest: Dest::parse(dest),
            jump: Jump::parse(jump),
        })
    }
}

impl fmt::Display for Comp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Comp::Zero => "0",
            Comp::One => "1",
            Comp::NegOne => "-1",
            Comp::D => "D",
            Comp::A => "A",
            Comp::NotD => "!D",
            Comp::NotA => "!A",
            Comp::NegD => "-D",
            Comp::NegA => "-A",
            Comp::DPlusOne => "D+1",
            Comp::APlusOne => "A+1",
            Comp::DMinusOne => "D-1",
            Comp::AMinusOne => "A-1",
            Comp::DPlusA => "D+A",
            Comp::DMinusA => "D-A",
            Comp::AMinusD => "A-D",
            Comp::DAndA => "D&A",
            Comp::DOrA => "D|A",
            Comp::M => "M",
            Comp::NotM => "!M",
            Comp::NegM => "-M",
            Comp::MPlusOne => "M+1",
            Comp::MMinusOne => "M-1",
            Comp::DPlusM => "D+M",
            Comp::DMinusM => "D-M",
            Comp::MMinusD => "M-D",
            Comp::DAndM => "D&M",
            Comp::DOrM => "D|M",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Dest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Dest::M => "M",
            Dest::D => "D",
            Dest::DM => "DM",
            Dest::A => "A",
            Dest::AM => "AM",
            Dest::AD => "AD",
            Dest::ADM => "ADM",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Jump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Jump::JGT => "JGT",
            Jump::JEQ => "JEQ",
            Jump::JGE => "JGE",
            Jump::JLT => "JLT",
            Jump::JNE => "JNE",
            Jump::JLE => "JLE",
            Jump::JMP => "JMP",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::A(val) => {
                let res = match *val {
                    SCREEN_ADDR => "@SCREEN".to_string(),
                    KBD_ADDR => "@KBD".to_string(),
                    v if v < PREDEFINED_REGISTERS_NUM => format!("@R{v}"),
                    v => format!("@{v}"),
                };
                write!(f, "{}", res)
            }
            Instruction::C { comp, dest, jump } => {
                if let Some(d) = dest {
                    write!(f, "{:?}=", d)?;
                }
                write!(f, "{}", comp)?;
                if let Some(j) = jump {
                    write!(f, ";{:?}", j)?;
                }
                Ok(())
            }
        }
    }
}
