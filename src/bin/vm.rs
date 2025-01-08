use std::env;
use std::io::{BufReader, Read};
use std::path::Path;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use vm::vm::{Machine, Register};


pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}", x))?;	

    let mut reader = BufReader::new(file);



    let mut bom = [0u8; 2];
    reader.read_exact(&mut bom).ok();
    if bom != [0xFF, 0xFE] {
        reader.seek(SeekFrom::Start(0)).map_err(|x| format!("failed to seek: {}", x))?;
    }



    let mut program: Vec<u8> = Vec::new();
    reader.read_to_end(&mut program).map_err(|x| format!("failed to read : {}", x))?;


    let mut vm = Machine::new();
    vm.memory.load_from_vec(program, 0);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {} ", vm.get_register(Register::A));
    Ok(())
}
