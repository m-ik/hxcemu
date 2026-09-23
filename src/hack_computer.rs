use std::{fmt, path::Path};

use crate::{
    cpu::Cpu,
    ram::Ram,
    rom::{Rom, ROM_SIZE},
};

pub struct HackComputer {
    rom: Rom,
    ram: Ram,
    cpu: Cpu,
}

#[derive(Debug)]
pub enum HackComputerError {
    PcOutOfBounds { pc: u16, max: usize },
}

impl fmt::Display for HackComputerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HackComputerError::PcOutOfBounds { pc, max } => {
                write!(f, "PC={} exceeded ROM capacity ({})", pc, max)
            }
        }
    }
}

impl HackComputer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut rom = Rom::new();
        rom.load_program_from_file(path)?;

        Ok(HackComputer {
            rom,
            ram: Ram::new(),
            cpu: Cpu::new(),
        })
    }

    pub fn tick(&mut self, reset: bool) -> Result<(), HackComputerError> {
        if self.cpu.pc() >= ROM_SIZE as u16 {
            return Err(HackComputerError::PcOutOfBounds {
                pc: self.cpu.pc(),
                max: ROM_SIZE,
            });
        }
        let instr = self.rom.fetch(self.cpu.pc());
        let in_m = self.ram.read(self.cpu.a_reg());
        let (out_m, write_m, addr_m) = self.cpu.tick(instr, in_m, reset);
        if write_m {
            self.ram.write(addr_m, out_m);
        }

        Ok(())
    }

    pub fn pc(&self) -> u16 {
        self.cpu.pc()
    }

    pub fn a_reg(&self) -> u16 {
        self.cpu.a_reg()
    }

    pub fn d_reg(&self) -> u16 {
        self.cpu.d_reg()
    }

    pub fn ram(&self, addr: u16) -> u16 {
        self.ram.read(addr)
    }

    pub fn rom(&self, addr: u16) -> u16 {
        self.rom.fetch(addr)
    }
}
