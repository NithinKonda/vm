use std::path::Path;
use std::{env,  fs::File};

fn main() {
    let args : Vec<_> = env:args().collect();
    if args.len() != 2 {
        panic!("usage : {} <input> ", args[0]);
    }

    let input_file = &args[1];

    let file = match File::open(Path::new(&args[1])) {
        Err(e) => panic!("failed to open file : {}", e),
        Ok(file) => file,
    };
}