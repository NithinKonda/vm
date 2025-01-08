use std::{fmt::format, fs::File, io::{BufReader, Read}};
use std::env;
use std::path::Path;
use vm::vm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}", x))?;	

    let mut reader = BufReader::new(file);
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
