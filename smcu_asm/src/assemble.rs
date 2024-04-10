use crate::asm_info::AsmInfo;
use crate::architecture::Architecture;
use crate::format::Format;
use crate::machine::Instruction;

pub fn assemble<'a, Arch: Architecture, F: Format<'a, Arch>>(asm: &mut AsmInfo, arch: &Arch, format: &F) -> Result<String, ()> {
    let prog_size = asm.instructions.len() + if arch.support_immidiates() { 0 } else { asm.immidiate_table.len() };

    if prog_size > arch.size() {
        return Err(())
    }

    if !arch.support_immidiates() {
        let size = arch.size();
        asm.instructions.reserve(arch.size());
        for _ in asm.instructions.len()..size {
            asm.instructions.push(Instruction::raw(0));
        }
        for (val, offset) in asm.immidiate_table.iter() {
            asm.instructions[size - (offset.to_owned() as usize) - 1] = Instruction::raw(val.to_owned());
        }
    }

    let mut buff = format.preamble();
    buff.push_str(&format.body(&asm));
    buff.push_str(&format.epilogue());
    Ok(buff)
}
