use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Imp = 0x98,
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
        (Opcode::Imp.into(), Instruction::from(Cpu::tya)),
    ]);
}

impl crate::cpu::Cpu{
    fn tya(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.a = self.y;
            }
            _ => {}
        }
        self.ps.set(CpuStatusFlags::Z, self.a == 0);
        self.ps.set(CpuStatusFlags::N, (self.a as SByte) <= 0);
    }
}