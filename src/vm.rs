mod vm;

enum Registers {
    A,B,C,M,SP,PC,BP,FLAGS
}




trait Addressable {
    fn read(addr : u16) -> Option<u8>;
    fn write(addr : u16, value : u8) -> bool;

    fn read2(addr: u16) -> Option<u16> {

    }

    fn write2(addr: u16, value: u16) -> bool {

    }


    fn copy(from : u16, to : u16) -> bool {
        
    }
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

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            self.step()?;
        }
    }
}