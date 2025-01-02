use vm::vm::Machine;

pub fn main() -> Result<(),String> {
    let mut vm = Machine::new();
    vm.memory.write(0, 0x01);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()


}