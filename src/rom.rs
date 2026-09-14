use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

pub const ROM_SIZE: usize = 32768;

pub struct Rom {
    mem: Box<[u16; ROM_SIZE]>,
}

impl Rom {
    pub fn new() -> Self {
        Self {
            mem: Box::new([0u16; ROM_SIZE]),
        }
    }

    pub fn load_program<R: BufRead>(&mut self, reader: R) -> io::Result<()> {
        let mut address = 0;

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if address >= ROM_SIZE {
                return Err(Error::new(ErrorKind::InvalidData, "ROM size exceeded"));
            }

            let instr = u16::from_str_radix(trimmed, 2).map_err(|_| {
                Error::new(ErrorKind::InvalidData, format!("invalid code: '{trimmed}'"))
            })?;

            self.mem[address] = instr;
            address += 1;
        }

        Ok(())
    }

    pub fn load_program_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        self.load_program(BufReader::new(file))
    }

    pub fn fetch(&self, pc: u16) -> u16 {
        self.mem[pc as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_program_valid() {
        let asm = "\
            // Compute RAM[0] = 2 + 3
            0000000000000010
            0000000000000011
            1110000010010000
        ";

        let mut rom = Rom::new();
        let result = rom.load_program(asm.as_bytes());

        assert!(result.is_ok());
        assert_eq!(rom.fetch(0), 0b0000000000000010); // @2
        assert_eq!(rom.fetch(1), 0b0000000000000011); // @3
        assert_eq!(rom.fetch(2), 0b1110000010010000); // D=A
    }

    #[test]
    fn load_program_invalid() {
        let asm = "0000000000000012";
        let mut rom = Rom::new();

        let result = rom.load_program(asm.as_bytes());
        assert!(result.is_err());
    }
}
