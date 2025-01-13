

use crate::memory::{Addressable, LinearMemory};
use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug)]
pub enum Register {
    A,B,C,M,SP,PC,BP,FLAGS
}


impl Register {
    pub fn from_u8(v:u8) -> Option<Self> {
        match v {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::SP as u8 => Some(Register::SP),
            x if x == Register::PC as u8 => Some(Register::PC),
            x if x == Register::BP as u8 => Some(Register::BP),
            x if x == Register::FLAGS as u8 => Some(Register::FLAGS),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddRegister(Register, Register),
    Signal(u8),
}

impl Op {

    pub fn value(&self) -> u8 {
        
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }    
}




type SignalFunction = fn(&mut Machine) -> Result<(),String>;

pub struct Machine {
    registers : [u16; 8],
    pub halt : bool,
    signal_handler : HashMap<u8, SignalFunction>,
    pub memory : Box<dyn Addressable>,
}




    /// Given an instruction, returns the argument portion of the instruction.
    ///
    /// This is the top 8 bits of the instruction, shifted right by 8 bits so that
    /// they are in the lower 8 bits, and then cast to a u8.
    ///
    /// This is used by many of the parse functions to extract the argument from an
    /// instruction.

fn parse_instruction_arg(ins: u16) -> u8 {
((ins & 0xff00) >> 8) as u8
}


fn parse_instruction(ins:u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;
    match op {
         x if x == Op::Nop.value() as u8 =>  Ok(Op::Nop),
         x if x == Op::Push(0).value() => {
            let arg = parse_instruction_arg(ins);
            Ok(Op::Push(arg as u8))
         },
         x if x == Op::PopReg(Register::A).value() => {
            let reg = (ins&0xff00) >> 8;
            Register::from_u8(reg as u8)
            .ok_or(format!("unknown register 0x{:X}", reg))
            .map(|r| Op::PopReg(r))        }
         x if x == Op::AddStack.value() => {
            Ok(Op::AddStack)
         }
         x if x == Op::Signal(0).value() => {
            let arg = parse_instruction_arg(ins);
            Ok(Op::Signal(arg))
         }
        _ =>  Err(format!("Unknown operatorrrr 0x{:X}", op)),
        
    }
}



impl Machine {
    pub fn new() -> Self {
        Self {
            registers : [0;8],
            halt:false,
            signal_handler : HashMap::new(),
            memory :Box::new(LinearMemory::new(8 * 1024)),
        }

    }

    pub fn get_register(&self, r:Register) -> u16 {
        self.registers[r as usize]
    }



pub fn define_handler(&mut self, index:u8, f: SignalFunction) {
    self.signal_handler.insert(index,f);
}

    pub fn pop(&mut self) -> Result<u16,String> {
        let sp = self.registers[Register::SP as usize] - 2;
                if let Some(v) = self.memory.read2(sp) {
                    self.registers[Register::SP as usize] -=2;
                    Ok(v)
                }
                else {
                    Err(format!("Failed to read value from stack at address {:04x}", sp))
                }
    }

    pub fn push(&mut self, v:u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("Failed to write value {:02x} to stack at address {:04x}", v, sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).ok_or(format!("Failed to read instruction at address 0x{:X}", pc))?;
        self.registers[Register::PC as usize] = pc + 2;


        let op = parse_instruction(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => {
                self.push(v.into())
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
            Op::Signal(signal) => {
                let sig_fn = self.signal_handler.get(&signal).ok_or(format!("Unknown signal {}", signal))?;
                sig_fn(self)
            },
            
        }

   }
}