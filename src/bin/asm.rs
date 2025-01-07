use std::io;
use std::path::Path;
use std::{env,  fs::File};

fn main() {
    let args : Vec<_> = env:args().collect();
    if args.len() != 2 {
        panic!("usage : {} <input> ", args[0]);
    }

    let input_file = &args[1];

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}",x))?; 




    let output : Vec<u8 = Vec::new();
    for line in io::BufReader::new(file).lines() {
        for t in line.split(" ").filter(|x| x.len() ==0) {
            let b = u8 ::from_str_radix(t,16).map_err(|x| format!("failed to parse : {}",x))?;
            output.push(b);
        }
    }

    

}