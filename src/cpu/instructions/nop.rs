use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Imp = 0xEA,
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
        (Opcode::Imp.into(), Instruction::from(Cpu::nop)),
    ]);
}

impl crate::cpu::Cpu{
    fn nop(&mut self, _memory: &mut Memory) {
        // NOP still expects to occupy 2 places in memory hence, forcing an extra skip in program counter in first clock cycle
        match self.tcu {
            1 => {
                self.pc += 1;
            }
            _ => {}
        }
    }
}