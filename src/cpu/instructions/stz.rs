use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Zp = 0x64,
    Abs = 0x9C,
    AbsXIdx = 0x9E,
    ZpXIdx = 0x74,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Zp.into(), AddressingType::ZeroPage), 
        (Opcode::Abs.into(), AddressingType::Absolute),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexed),
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::stz)),
        (Opcode::Abs.into(), Instruction::from(Cpu::stz)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::stz)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::stz)),
    ]);
}

impl crate::cpu::Cpu{
    fn stz(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            memory.write_byte(self.addressing, 0x00);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            memory.write_byte(self.addressing, 0x00);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction STZ!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction STZ!");
        }
    }
}