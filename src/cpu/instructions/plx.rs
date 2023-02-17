use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Sta = 0xFA,
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
        (Opcode::Sta.into(), Instruction::from(Cpu::plx)),
    ]);
}

impl crate::cpu::Cpu{
    fn plx(&mut self, memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.x = self.stack_pull(memory);
            }
            _ => {}
        }
        self.ps.set(CpuStatusFlags::Z, self.x == 0);
        self.ps.set(CpuStatusFlags::N, (self.x as SByte) < 0);
    }
}