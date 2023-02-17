use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_byte_signed_byte}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Imp = 0xE8,
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
        (Opcode::Imp.into(), Instruction::from(Cpu::iny)),
    ]);
}

impl crate::cpu::Cpu{
    fn iny(&mut self, _memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.y = offset_byte_signed_byte(self.y, 0x01);
                self.alu = self.y;
            }
            _ => {}
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);
    }
}