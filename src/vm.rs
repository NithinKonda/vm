use crate::memory::{Addressable, LinearMemory};


#[derive(Debug)]
pub enum Register {
    A,B,C,M,SP,PC,BP,FLAGS
}

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddRegister(Register, Register),
}

impl Op {

    pub fn value(&self) -> u8 {
        
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }    
}



pub struct Machine {
    registers : [u16; 8],
    pub memory : Box<dyn Addressable>,
}



fn parse_instruction(ins:u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;
    match op {
         x if x == Op::Nop.value() as u8 =>  Ok(Op::Nop),
         x if x == Op::Push(0).value() => {
            let arg = (ins & 0xff00) >> 8;
            Ok(Op::Push(arg as u8))
         }
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



    pub fn pop(&mut self) -> Result<u16,String> {
        let sp = self.registers[Register::SP as usize] - 2;
                if let Some(v) = self.memory.read2(sp) {
                    Ok(v)
                }
                else {
                    Err(format!("Failed to read value from stack at address {:04x}", sp))
                }
    }

    pub fn push(&mut self, v:u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write(sp, v as u8) {
            return Err(format!("Failed to write value {:02x} to stack at address {:04x}", v, sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Register::PC as usize] = pc + 2;


        let op = parse_instruction(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => {
                self.push(v as u16)
           },

            Op::PopReg(r) => {
                let value = self.pop()?;
                self.registers[r as usize] = value;
                Ok(())
               },

                Op::AddStack => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(a + b)
                    
                },
                Op::AddRegister(r1, r2) => {
                    self.registers[r1 as usize] += self.registers[r2 as usize];
                   Ok(())

            }
            _ =>  Err(format!("Unknown operator {:?}", op)),
            
        }

        // println!("Executing instruction {:04x} at address {:04x}", instruction, pc);
        // Ok(())
    }
}