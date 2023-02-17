use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Imp = 0x18,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Imp.into(), AddressingType::Implied),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Imp.into(), Instruction::from(Cpu::clc)),
    ]);
}

impl crate::cpu::Cpu{
    fn clc(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.ps.set(CpuStatusFlags::C, false);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::setup_test_with_ps;
    use crate::cpu::{Byte, CpuStatusFlags, instructions::clc};

    #[test]
    fn test_operation() {
        let (cpu, _) =
            setup_test_with_ps(
                None,
                None,
                None,
                None,
                CpuStatusFlags::C,
                clc::Opcode::Imp as Byte,
                0x00
            );
        cpu.assert_status(CpuStatusFlags::C, false);
    }
}