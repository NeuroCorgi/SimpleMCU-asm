use std::collections::HashMap;
use crate::machine;

pub struct AsmInfo {
    pub instructions:    Vec<machine::Instruction>,
    pub symbol_table:    HashMap<String, u8>,
    pub immidiate_table:    HashMap<i8, u8>,
}

impl AsmInfo {
    pub fn new() -> AsmInfo {
        AsmInfo {
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            immidiate_table: HashMap::new(),
        }
    }
}
