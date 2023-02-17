use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_word_signed_byte}, Cpu}, memory::Memory};

pub enum Opcode {
    PcRel = 0x80,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::PcRel.into(), AddressingType::PcRelative),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::PcRel.into(), Instruction::from(Cpu::bra)),
    ]);
}

impl crate::cpu::Cpu{
    fn bra(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.pc = offset_word_signed_byte(self.pc, self.alu);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::{Byte, instructions::bra, SByte}, test_utils::setup_test};

    const OFFSET: Byte = 2; 

    #[test]
    fn test_operation() {
        let (cpu, _) =
            setup_test(
                None,
                None,
                None,
                None,
                bra::Opcode::PcRel as Byte,
                (0 - OFFSET as SByte) as Byte
            );
        cpu.assert_pc(crate::test_utils::START_PROGRAM);
    }
}