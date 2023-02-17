use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Abs = 0x4C,
    AbsXIdx = 0x7C,
    AbsInd = 0x6C,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Abs.into(), AddressingType::Absolute),
        (Opcode::AbsInd.into(), AddressingType::AbsoluteIndirect),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Abs.into(), Instruction::from(Cpu::jmp)),
        (Opcode::AbsInd.into(), Instruction::from(Cpu::jmp)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::jmp)),
    ]);
}

impl crate::cpu::Cpu{
    fn jmp(&mut self, _memory: &mut Memory) {
        // Addessing sets up program counter on other addressing modes
        if self.ir == Opcode::Abs.into() {
            match self.tcu {
                2 => {
                    self.pc = self.addressing
                }
                _ => {}
            }
        }
    }
}