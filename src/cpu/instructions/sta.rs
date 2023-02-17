use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0x81,
    Zp = 0x85,
    Abs = 0x8D,
    ZpIndYIdx = 0x91,
    ZpInd = 0x92,
    ZpXIdx = 0x95,
    AbsYIdx = 0x99,
    AbsXIdx = 0x9D
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::ZpXIdxInd.into(), AddressingType::ZeroPageXIndexedIndirect),
        (Opcode::Zp.into(), AddressingType::ZeroPage), 
        (Opcode::Abs.into(), AddressingType::Absolute),
        (Opcode::ZpIndYIdx.into(), AddressingType::ZeroPageIndirectYIndexed),
        (Opcode::ZpInd.into(), AddressingType::ZeroPageIndirect),
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexed),
        (Opcode::AbsYIdx.into(), AddressingType::AbsoluteYIndexed),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::sta)),
        (Opcode::Zp.into(), Instruction::from(Cpu::sta)),
        (Opcode::Abs.into(), Instruction::from(Cpu::sta)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::sta)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::sta)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::sta)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::sta)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::sta)),
    ]);
}

impl crate::cpu::Cpu{
    fn sta(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            memory.write_byte(self.addressing, self.a);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            memory.write_byte(self.addressing, self.a);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPageXIndexedIndirect => {
                    match self.tcu {
                        4 => {
                            memory.write_byte(self.addressing, self.a);
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPageIndirect | AddressingType::ZeroPageIndirectYIndexed => {
                    match self.tcu {
                        3 => {
                            memory.write_byte(self.addressing, self.a);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction STA!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction STA!");
        }
    }
}