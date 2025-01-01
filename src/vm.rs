
enum Registers {
    A,B,C,M,SP,PC,BP,FLAGS
}




trait Addressable {
    fn read(&self, addr : u16) -> Option<u8>;
    fn write(&mut self, addr : u16, value : u8) -> bool;

    fn read2(&mut self, addr: u16) -> Option<u16> {

        if let Some(x0) = self.read(addr) {
            if let Some(x1) = self.read(addr+1) {
                return (x0 as u16) | ((x1 as u16) << 8);
            } 
        };
        None
    }

    fn write2(addr: u16, value: u16) -> bool {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;

        self.write(addr, lower) && self.write(addr+1, upper)
    }


    fn copy(from : u16, to : u16, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read(from+i) {
                if !self.write(to+i, x) {
                    return false;
                }
            } else {
                return false;
            }
        }        
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