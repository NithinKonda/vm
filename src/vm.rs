use crate::memory::{Addressable, LinearMemory};
pub enum Registers {
    A,B,C,M,SP,PC,BP,FLAGS
}

#[repr(u8)]
pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddRegister(Register, Register),
}

impl Op {

    pub fn value(&self) -> u8 {
        unsafe {*<*const_>::from(self).cast::<u8>()}
    }    


}



pub struct Machine {
    registers : [u16; 8],
    pub memory : Box<dyn Addressable>,
}



fn parse_instruction(x:u16) -> Result<Instruction, String> {
    let op = (x & 0xff) as u8;
    match op {
        x if x == Op::Nop.value() as u8 =>  Ok(Instruction::Nop),
        _ =>  Err(format!("Unknown operator 0x{:X}", op)),
        
    }
}



impl Machine {
    pub fn new() -> Self {
        Self {
            registers : [0;8],
            memory :Box::new(LinearMemory::new(8 * 1024)),
        }

    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Registers::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Registers::PC as usize] = pc + 2;


        let op = (instruction & 0xff) as u8;
        match op {
            x if x == Op::Nop as u8 =>  Ok(()),
            _ =>  Err(format!("Unknown operator 0x{:X}", op)),
            
        }

        // println!("Executing instruction {:04x} at address {:04x}", instruction, pc);
        // Ok(())
    }
}