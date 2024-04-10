use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Assembling: {filename}");
    let contents = fs::read_to_string(filename).unwrap();

    let assembled = smcu_asm::run(contents);
    // println!("Hello, world!");
    println!("{assembled}");
    
}
