use std::{fmt::format, fs::File, intrinsics::mir::Len};

use vm::vm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args : Vec<_> = env::args().collect();

    let args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}", x))?;	
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {} ", vm.get_register(Register::A));
    Ok(())
}
