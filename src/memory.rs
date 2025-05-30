pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 4096] }
    }

    pub fn store(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
}
