mod machine;
mod asm_info;
pub mod architecture;
pub mod format;

pub mod parser;
pub mod assemble;

pub fn run(input: String) -> String {
    let mut asm = parser::parse(&input).expect("");
    let arch = architecture::Default{};
    let format: format::Mif<_> = format::Format::new(&arch);

    match assemble::assemble(&mut asm, &arch, &format) {
        Ok(res) => res,
        Err(e) => {
            println!("{:?}", e);
            String::from("Some error")
        }
    }
}
