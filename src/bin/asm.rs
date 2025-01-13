use std::io::{self,  Write};
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::{env,  fs::File};

fn main() -> Result<(), String> {
    let args : Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage : {} <input> ", args[0]);
    }

    let _input_file = &args[1];

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open : {}",x))?; 




    let mut output: Vec<u8> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_inner = line.map_err(|x | format!("failed to read line : {}",x) )?;
        for t in line_inner.split(" ").filter(|x| x.len() > 0) {
            let b = u8 ::from_str_radix(t,16).map_err(|x| format!("failed to parse : {}",x))?;
            output.push(b);
        }
    }


    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(| x | format!("failed to write : {}",x))?;
    Ok(())    

}