use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_byte_signed_byte}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Imp = 0xCA,
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
        (Opcode::Imp.into(), Instruction::from(Cpu::dex)),
    ]);
}

impl crate::cpu::Cpu{
    fn dex(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.x = offset_byte_signed_byte(self.x, 0xFF);
                self.alu = self.x;
            }
            _ => {}
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);
    }
}