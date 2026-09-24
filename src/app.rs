use crate::hack_computer::HackComputer;

pub struct App<'a> {
    pub a_reg: u16,
    pub d_reg: u16,
    pub pc: u16,
    pub program_path: &'a str,
    pub hack_computer: &'a HackComputer,
}

impl<'a> App<'a> {
    pub fn new(hack_computer: &'a HackComputer) -> Self {
        App {
            a_reg: hack_computer.a_reg(),
            d_reg: hack_computer.d_reg(),
            pc: hack_computer.pc(),
            program_path: hack_computer.program_path(),
            hack_computer,
        }
    }
}
