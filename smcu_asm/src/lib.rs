mod machine;
mod asm_info;
mod architecture;
mod format;

pub mod parser;
pub mod assemble;

#[derive(Debug)]
pub enum AssembleError {
    ParseError(parser::ParseError),
    AssembleError,
}

impl From<parser::ParseError> for AssembleError {
    fn from(value: parser::ParseError) -> Self {
        AssembleError::ParseError(value)
    }
}

pub fn run(input: String) -> Result<String, Vec<AssembleError>> {
    let mut asm = match parser::parse(&input) {
        Ok(info) => info,
        Err(errors) => {
            let errors: Vec<AssembleError> = errors.into_iter().map(AssembleError::from).collect();
            return Err(errors)
        }
    };

    let arch = architecture::Default{};
    let format: format::Mif<_> = format::Format::new(&arch);

    match assemble::assemble(&mut asm, &arch, &format) {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("{:?}", e);
            Err(vec![AssembleError::AssembleError])
        }
    }
}
