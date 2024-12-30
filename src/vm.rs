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
}