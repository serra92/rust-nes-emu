use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Zp = 0x86,
    Abs = 0x8E,
    ZpYIdx = 0x96,
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
        (Opcode::ZpYIdx.into(), AddressingType::ZeroPageYIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::stx)),
        (Opcode::Abs.into(), Instruction::from(Cpu::stx)),
        (Opcode::ZpYIdx.into(), Instruction::from(Cpu::stx)),
    ]);
}

impl crate::cpu::Cpu{
    fn stx(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::ZeroPageYIndexed => {
                    match self.tcu {
                        2 => {
                            memory.write_byte(self.addressing, self.x);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            memory.write_byte(self.addressing, self.x);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction STX!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction STX!");
        }
    }
}