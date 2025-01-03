use vm::vm::{Machine,Register};

pub fn main() -> Result<(),String> {
    let mut vm = Machine::new();
    vm.memory.write(0, 0x01);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;


println!("A = {} ", vm.get_register(Register::A));
Ok(())
    


}