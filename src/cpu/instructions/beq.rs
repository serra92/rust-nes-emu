use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_word_signed_byte}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    PcRel = 0xF0,
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
        (Opcode::PcRel.into(), Instruction::from(Cpu::beq)),
    ]);
}

impl crate::cpu::Cpu{
    fn beq(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                if self.ps.contains(CpuStatusFlags::Z) {
                    self.pc = offset_word_signed_byte(self.pc, self.alu);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::setup_test_with_ps, cpu::{Byte, CpuStatusFlags, Word, instructions::beq, SByte}};
    const OFFSET: Byte = 2;
    #[test]

    fn test_zero_set() {
        let (cpu, _) =
            setup_test_with_ps(
                None,
                None,
                None,
                None,
                CpuStatusFlags::Z,
                beq::Opcode::PcRel as Byte,
                (0 - OFFSET as SByte) as Byte
            );
        cpu.assert_pc(crate::test_utils::START_PROGRAM);
    }

    #[test]
    fn test_zero_clear() {
        let (cpu, _) =
            setup_test_with_ps(
                None,
                None,
                None,
                None,
                CpuStatusFlags::empty(),
                beq::Opcode::PcRel as Byte,
                (0 - OFFSET as SByte) as Byte
            );
        cpu.assert_pc(crate::test_utils::START_PROGRAM + Word::from(OFFSET));
    }
}