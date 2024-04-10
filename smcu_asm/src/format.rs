use crate::{
    architecture::Architecture,
    asm_info::AsmInfo,
    machine::{Instruction, Address, instruction::InstructionType}
};

pub trait Format<'a, Arch: Architecture> {
    type Word;

    fn new(arch: &'a Arch) -> Self;

    fn preamble(&self) -> String;
    
    fn body(&self, info: &AsmInfo) -> String;
    
    fn epilogue(&self) -> String;
}

pub struct Mif<'a, Arch: Architecture> { architecture: &'a Arch, body: Vec<String>, }
pub struct Binary<'a, Arch: Architecture> { architecture: &'a Arch }

impl<'a, Arch: Architecture> Format<'a, Arch> for Mif<'a, Arch> {
    type Word = String;

    fn new(architecture: &'a Arch) -> Self {
        let body = Vec::with_capacity(architecture.size());
        Mif { architecture, body }
    }

    fn preamble(&self) -> String {
        format!("\
WIDTH={};
DEPTH={};

ADDRESS_RADIX=UNS;
DATA_RADIX=BIN;

CONTENT BEGIN
",
            self.architecture.width(),
            self.architecture.size()
        )
    }

    fn body(&self, info: &AsmInfo) -> String {
        info.instructions.iter()
            .enumerate()
            .map(|(i, Instruction { opcode, arg }) | {
                let op = match opcode {
                    InstructionType::Nor => "01",
                    InstructionType::Add => "10",
                    InstructionType::Sta => "11",
                    InstructionType::Jcc => "00",
                    InstructionType::Raw => "",
                };
                let arg = match arg {
                    Address::Immidiate(addr) if opcode == &(InstructionType::Raw)
                        => format!("{:08b}", addr),
                    Address::Immidiate(val) => {
                        let offset = info.immidiate_table[val] + 1;
                        format!("{:06b}", (self.architecture.size() as u8) - offset)
                    },
                    Address::Symbol(symb) => format!("{:06b}", info.symbol_table[symb]),
                    Address::Memory(addr) => format!("{:06b}", addr),
                };
                format!("{i} : {}{}\n", op, arg)
            })
            .collect::<String>()
    }

    fn epilogue(&self) -> String {
        String::from("END;")
    }
}

impl<'a, Arch: Architecture> Format<'a, Arch> for Binary<'a, Arch> {
    type Word = u8;

    fn new(architecture: &'a Arch) -> Self {
        Binary { architecture }
    }

    fn preamble(&self) -> String {
        String::from("")
    }

    fn body(&self, _info: &AsmInfo) -> String {
        String::from("")
    }

    fn epilogue(&self) -> String {
        String::from("")
    }
}
