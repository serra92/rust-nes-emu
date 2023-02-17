use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, Word}, memory::Memory};

pub enum Opcode {
    Sta = 0x60,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Sta.into(), AddressingType::SubroutineReturn),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Sta.into(), Instruction::from(Cpu::rts)),
    ]);
}

impl crate::cpu::Cpu{
    fn rts(&mut self, memory: &mut Memory) {
        match self.tcu {
            1 => {
                self.pc = Word::from(self.stack_pull(memory));
            }
            2 => {
                self.pc = Word::from(self.stack_pull(memory)) << 8;
            }
            3 => {
                self.pc += 1;
            }
            _ => {}
        }
    }
}