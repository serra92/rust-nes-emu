use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Sta = 0x7A,
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
        (Opcode::Sta.into(), Instruction::from(Cpu::ply)),
    ]);
}

impl crate::cpu::Cpu{
    fn ply(&mut self, memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.y = self.stack_pull(memory);
            }
            _ => {}
        }
        self.ps.set(CpuStatusFlags::Z, self.y == 0);
        self.ps.set(CpuStatusFlags::N, (self.y as SByte) < 0);
    }
}