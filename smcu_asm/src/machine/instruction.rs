use super::Address;

#[derive(PartialEq)]
pub enum InstructionType {
    Nor,
    Add,
    Sta,
    Jcc,
    Raw,
}

pub struct Instruction {
    pub opcode: InstructionType,
    pub arg: Address,
}

impl Instruction {
    pub fn nor(address: Address) -> Instruction {
        Instruction {
            opcode: InstructionType::Nor,
            arg: address,
        }
    }

    pub fn add(address: Address) -> Instruction {
        Instruction {
            opcode: InstructionType::Add,
            arg: address
        }
    }

    pub fn sta(address: Address) -> Instruction {
        Instruction {
            opcode: InstructionType::Sta,
            arg: address
        }
    }

    pub fn jcc(address: Address) -> Option<Instruction> {
        match address {
            Address::Immidiate(_) => None,
            _ => Some(Instruction {
                opcode: InstructionType::Jcc,
                arg: address,
            })
        }
    }

    pub fn raw(val: i8) -> Instruction {
        Instruction {
            opcode: InstructionType::Raw,
            arg: Address::Immidiate(val),
        }
    }
}

#[derive(Debug)]
pub enum InstructionParseError {
    InvalidInstructionFormat,
    UnknownInstruction,
    WrongAddressTypeForJump(Address),
}


impl TryFrom<&str> for Instruction {
    type Error = InstructionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use InstructionParseError::*;

        let parse: Vec<&str> = value.trim().splitn(3, ' ').collect();
        if parse.len() < 2 {
            return Err(InvalidInstructionFormat);
        }
        
        let opcode = parse[0];
        let arg = Address::from(parse[1]);
        match &opcode.to_lowercase()[..] {
            "nor" => {
                Ok(Instruction::nor(arg))
            },
            "add" => {
                Ok(Instruction::add(arg))
            },
            "sta" => {
                Ok(Instruction::sta(arg))
            },
            "jcc" => {
                if let Some(jump) = Instruction::jcc(arg) {
                    Ok(jump)
                } else {
                    // Let's take into account that I'm not a Rust developer
                    // And I don't know of better way to do it, yet
                    let arg = Address::from(parse[1]);
                    Err(WrongAddressTypeForJump(arg))
                }
            }
            _ => Err(UnknownInstruction)
        }
    }
}
