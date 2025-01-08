use std::{fmt::format, fs::File, intrinsics::mir::Len, io::{BufReader, Read}};

use vm::vm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args : Vec<_> = env::args().collect();

    let args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}", x))?;	

    let reader = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();
    reader.read_to_end(program);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {} ", vm.get_register(Register::A));
    Ok(())
}
