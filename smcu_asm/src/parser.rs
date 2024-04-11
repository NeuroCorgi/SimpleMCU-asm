use std::collections::hash_map::Entry;

use crate::machine::{Address, Instruction};
pub use crate::machine::InstructionParseError;
use crate::asm_info::AsmInfo;

#[derive(Debug)]
pub enum DerectiveError {
    InvalidArgument(String),
    UnknownDerective(String),
}

#[derive(Debug)]
pub enum SymbolError {
    UnknownSymbol(String),
    RedefinitionOfSymbol(String),
}

#[derive(Debug)]
pub enum ParseError {
    DerectiveError(DerectiveError),
    SymbolError(SymbolError),
    InstructionError(InstructionParseError),
}

pub fn parse(file: &String) -> Result<AsmInfo, Vec<ParseError>> {
    let mut asm = AsmInfo::new();
    let mut errors: Vec<ParseError> = vec![];

    let mut location_counter = 0;
    let mut immidiate_counter = 0;
    
    for line in file.lines().map(str::trim) {
        if line.starts_with('.') { // Derective

            let parse: Vec<&str> = line.trim().splitn(3, ' ').collect();
            
            let derective = parse[0];
            let arg = parse[1];
            match derective {
                ".org" => {
                    if let Ok(len) = arg.parse::<u8>() {
                        while location_counter < len {
                            asm.instructions.push(Instruction::raw(0));
                            location_counter += 1;
                        }
                    } else {
                        let e = DerectiveError::InvalidArgument(String::from(arg));
                        errors.push(ParseError::DerectiveError(e));
                    }
                },
                ".byte" => {
                    if let Ok(val) = arg.parse::<i8>() {
                        asm.instructions.push(Instruction::raw(val));
                    } else {
                        let e = DerectiveError::InvalidArgument(String::from(arg));
                        errors.push(ParseError::DerectiveError(e));
                    }
                    location_counter += 1;
                },
                _ => {
                    let e = DerectiveError::UnknownDerective(String::from(derective));
                    errors.push(ParseError::DerectiveError(e));
                }
            }
            
        } else if line.ends_with(':') { // Symbol
            // Cut colon at the end of the symbol
            // Possibly panics
            let raw_symbol = &line[..line.len() - 1];
            
            let symbol = String::from(raw_symbol);
            match asm.symbol_table.entry(symbol) {
                Entry::Occupied(_) => {
                    let symbol = String::from(raw_symbol);
                    let e = SymbolError::RedefinitionOfSymbol(symbol);
                    errors.push(ParseError::SymbolError(e));
                },
                Entry::Vacant(e) => {
                    e.insert(location_counter);
                }
            }
            
        } else if !line.is_empty() { // Opcode
            match Instruction::try_from(line) {
                Ok(instr) => {
                    match instr.arg {
                        Address::Immidiate(val) => {
                            asm.immidiate_table.entry(val)
                                .or_insert_with(|| {
                                    immidiate_counter += 1;
                                    immidiate_counter - 1
                                });
                        },
                        _ => {}
                    }
                    asm.instructions.push(instr)
                },
                Err(e) => errors.push(ParseError::InstructionError(e)),
            }
            location_counter += 1;
        }
    }

    asm.instructions.iter().for_each(
        |Instruction { opcode: _, arg }| {
            if let Address::Symbol(s) = arg {
                if !asm.symbol_table.contains_key(s) {
                    let e = SymbolError::UnknownSymbol(s.clone());
                    errors.push(ParseError::SymbolError(e));
                }
            }
        });
    
    if errors.is_empty() {
        Ok(asm)
    } else {
        Err(errors)
    }
}
