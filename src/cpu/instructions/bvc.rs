use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_word_signed_byte}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    PcRel = 0x50,
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
        (Opcode::PcRel.into(), Instruction::from(Cpu::bvc)),
    ]);
}

impl crate::cpu::Cpu{
    fn bvc(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                if !self.ps.contains(CpuStatusFlags::V) {
                    self.pc = offset_word_signed_byte(self.pc, self.alu);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::setup_test_with_ps, cpu::{Byte, CpuStatusFlags, Word, instructions::bvc, SByte}};
    const OFFSET: Byte = 2;
    #[test]

    fn test_overflow_set() {
        let (cpu, _) =
            setup_test_with_ps(
                None,
                None,
                None,
                None,
                CpuStatusFlags::V,
                bvc::Opcode::PcRel as Byte,
                (0 - OFFSET as SByte) as Byte
            );
        cpu.assert_pc(crate::test_utils::START_PROGRAM + Word::from(OFFSET));
    }

    #[test]
    fn test_overflow_clear() {
        let (cpu, _) =
            setup_test_with_ps(
                None,
                None,
                None,
                None,
                CpuStatusFlags::empty(),
                bvc::Opcode::PcRel as Byte,
                (0 - OFFSET as SByte) as Byte
            );
        cpu.assert_pc(crate::test_utils::START_PROGRAM);
    }
}