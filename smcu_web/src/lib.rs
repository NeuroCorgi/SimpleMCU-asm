use wasm_bindgen::prelude::*;

use smcu_asm::{
    // self,
    parser::{ParseError, DerectiveError, SymbolError, InstructionParseError},
    AssembleError,
};

fn convert(err: AssembleError) -> String {
    match err {
        AssembleError::ParseError(ParseError::DerectiveError(DerectiveError::InvalidArgument(e))) => format!("Invalid argument for derective: {}", e),
        AssembleError::ParseError(ParseError::DerectiveError(DerectiveError::UnknownDerective(e))) => format!("Unknown derective: {}", e),
        AssembleError::ParseError(ParseError::SymbolError(SymbolError::UnknownSymbol(e))) => format!("Unknown symbol: {}", e),
        AssembleError::ParseError(ParseError::SymbolError(SymbolError::RedefinitionOfSymbol(e))) => format!("Redefinition of symbol: {}", e),
        AssembleError::ParseError(ParseError::InstructionError(InstructionParseError::InvalidInstructionFormat)) => format!("Invalid instruction format"),
        AssembleError::ParseError(ParseError::InstructionError(InstructionParseError::UnknownInstruction)) => format!("Unknown instruction"),
        AssembleError::ParseError(ParseError::InstructionError(InstructionParseError::WrongAddressTypeForJump(_))) => format!("Wrong address type for jump"),
        AssembleError::AssembleError => format!("Failed to assemble"),
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum ResultType {
    Ok,
    Err,
}

#[wasm_bindgen(getter_with_clone)]
pub struct Result {
    pub data: Vec<String>,
    pub res: ResultType,
}

#[wasm_bindgen]
pub fn run(input: String) -> Result {
    match smcu_asm::run(input) {
        Ok(res) => Result {
            data: vec![res],
            res: ResultType::Ok,
        },
        Err(errors) => {
            let errors: Vec<String> = errors.into_iter().map(convert).collect();
            Result {
                data: errors,
                res: ResultType::Err,
            }
        }
    }
}

