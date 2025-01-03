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

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Register::PC as usize] = pc + 2;


        let op = parse_instruction(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => {
                let sp = self.registers[Register::SP as usize];
                if !self.memory.write(sp, v) {
                    return Err(format!("Failed to write value {:02x} to stack at address {:04x}", v, sp));
                }
                self.registers[Register::SP as usize] += 2;
                Ok(())
            },
            Op::PopReg(r) => {
                let sp = self.registers[Register::SP as usize] - 2;
                if let Some(v) = self.memory.read2(sp) {
                    self.registers[r as usize] = v;
                    self.registers[Register::SP as usize] -= 2;
                    Ok(())
                }
                else {
                    Err(format!("Failed to read value from stack at address {:04x}", sp))
                }
            }
            _ =>  Err(format!("Unknown operator {:?}", op)),
            
        }

        // println!("Executing instruction {:04x} at address {:04x}", instruction, pc);
        // Ok(())
    }
}