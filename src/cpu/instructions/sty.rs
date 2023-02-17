use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    Zp = 0x84,
    Abs = 0x8C,
    ZpXIdx = 0x94,
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
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::sty)),
        (Opcode::Abs.into(), Instruction::from(Cpu::sty)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::sty)),
    ]);
}

impl crate::cpu::Cpu{
    fn sty(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            memory.write_byte(self.addressing, self.y);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            memory.write_byte(self.addressing, self.y);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction STY!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction STY!");
        }
    }
}