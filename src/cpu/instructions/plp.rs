use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Sta = 0x28,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Sta.into(), AddressingType::StackPull),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Sta.into(), Instruction::from(Cpu::plp)),
    ]);
}

impl crate::cpu::Cpu{
    fn plp(&mut self, memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.ps = CpuStatusFlags { bits: self.stack_pull(memory) };
            }
            _ => {}
        }
    }
}