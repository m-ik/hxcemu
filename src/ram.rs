const RAM_SIZE: usize = 24576;

pub struct Ram {
    mem: Box<[u16; RAM_SIZE]>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            mem: Box::new([0u16; RAM_SIZE]),
        }
    }

    pub fn read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }
}
