enum Registers {
    A,B,C,M,SP,PC,BP,FLAGS
}


struct Machine {
    registers : [u16; 8],
    memory : [u8; 5000],
}

impl Machine {
    pub fn new() -> Self {
        Self {
            register : [0;8],
            memory : [0;5000],
        }
    }

    pub fn step(&mut self) -> Result<(),&'static str> {
        let pc = self.registers[Registers::PC];
        self.memory.read
    }
}