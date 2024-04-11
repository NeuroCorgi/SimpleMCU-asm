use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Assembling: {filename}");
    let contents = fs::read_to_string(filename).unwrap();

    match smcu_asm::run(contents) {
        Ok(assembled) => println!("{assembled}"),
        Err(errors) => errors.iter().for_each(|e| {dbg!(e);}),
    }
    // println!("Hello, world!");
    
}
