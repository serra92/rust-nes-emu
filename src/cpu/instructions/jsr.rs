use std::collections::HashMap;

use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Abs = 0x4C,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Abs.into(), AddressingType::SubroutineJump),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Abs.into(), Instruction::from(Cpu::jsr)),
    ]);
}

impl crate::cpu::Cpu{
    fn jsr(&mut self, memory: &mut Memory) {
        let mut pc_buff: [Byte; 2] = [0,0];
        LittleEndian::write_u16(&mut pc_buff, self.pc - 1);
        match self.tcu {
            2 => {
                self.stack_push(memory, pc_buff[1]);
            }
            3 => {
                self.stack_push(memory, pc_buff[0]);
            }
            4 => {
                self.pc = self.addressing;
            }
            _ => {}
        }
    }
}