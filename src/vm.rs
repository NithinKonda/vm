use crate::memory::{Addressable, LinearMemory};
pub enum Registers {
    A,B,C,M,SP,PC,BP,FLAGS
}





pub struct Machine {
    registers : [u16; 8],
    memory : Box<dyn Addressable>,
}




impl Machine {
    pub fn new() -> Self {
        Self {
            registers : [0;8],
            memory :Box::new(LinearMemory::new(8 * 1024)),
        }

    }

    pub fn step(&mut self) -> Result<(),&'static str> {
        let pc = self.registers[Registers::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        println!("Executing instruction {:04x} at address {:04x}", instruction, pc);
        Ok(())
    }
}