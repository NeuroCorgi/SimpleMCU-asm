use crate::machine::Instruction;

pub trait Architecture {
    fn width(&self) -> usize;
    fn size(&self) -> usize;

    fn support_immidiates(&self) -> bool;
}

pub struct Default {}

impl Architecture for Default {
    fn width(&self) -> usize { 8 }
    fn size(&self)  -> usize { 64 }
    fn support_immidiates(&self) -> bool { false }
}

