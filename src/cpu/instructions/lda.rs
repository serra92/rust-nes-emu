use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0xA1,
    Zp = 0xA5,
    Imm = 0xA9,
    Abs = 0xAD,
    ZpIndYIdx = 0xB1,
    ZpInd = 0xB2,
    ZpXIdx = 0xB5,
    AbsYIdx = 0xB9,
    AbsXIdx = 0xBD
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
        (Opcode::Imm.into(), AddressingType::Immediate), 
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::lda)),
        (Opcode::Zp.into(), Instruction::from(Cpu::lda)),
        (Opcode::Imm.into(), Instruction::from(Cpu::lda)),
        (Opcode::Abs.into(), Instruction::from(Cpu::lda)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::lda)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::lda)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::lda)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::lda)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::lda)),
    ]);
}

impl crate::cpu::Cpu{
    fn lda(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        2 => {
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPageXIndexedIndirect => {
                    match self.tcu {
                        4 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        5 => {
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPageIndirect | AddressingType::ZeroPageIndirectYIndexed => {
                    match self.tcu {
                        3 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        4 => {
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction LDA!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction LDA!");
        }
        self.ps.set(CpuStatusFlags::Z, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::N, self.alu == 0);
    }
}