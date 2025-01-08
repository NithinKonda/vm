use std::intrinsics::mir::Len;

use vm::vm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args : Vec<_> = env::args().collect();

    let args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {} ", vm.get_register(Register::A));
    Ok(())
}
