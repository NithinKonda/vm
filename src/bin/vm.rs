use vm::vm::Machine;

pub fn main() -> () {
    let mut vm = Machine::new();
    vm.step();
}