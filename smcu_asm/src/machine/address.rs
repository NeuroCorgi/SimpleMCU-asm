#[derive(Debug)]
pub enum Address {
    Symbol(String),
    Memory(u8),
    Immidiate(i8),
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        if value.starts_with('#') {
            let value: i8 = value[1..].parse().unwrap();
            Address::Immidiate(value)
        } else if value.starts_with('[') && value.ends_with(']') {
            let address: u8 = value[1..value.len() - 1].parse().unwrap();
            Address::Memory(address)
        } else {
            Address::Symbol(String::from(value))
        }
    }
}
