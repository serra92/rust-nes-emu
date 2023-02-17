use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0x41,
    Zp = 0x45,
    Imm = 0x49,
    Abs = 0x4D,
    ZpIndYIdx = 0x51,
    ZpInd = 0x52,
    ZpXIdx = 0x55,
    AbsYIdx = 0x59,
    AbsXIdx = 0x5D
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::eor)),
        (Opcode::Zp.into(), Instruction::from(Cpu::eor)),
        (Opcode::Imm.into(), Instruction::from(Cpu::eor)),
        (Opcode::Abs.into(), Instruction::from(Cpu::eor)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::eor)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::eor)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::eor)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::eor)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::eor)),
    ]);
}

impl crate::cpu::Cpu{
    fn eor(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.alu ^= self.a;
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.alu ^= self.a;
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
                            self.alu ^= self.a;
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
                            self.alu ^= self.a;
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
                            self.alu ^= self.a;
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction EOR!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction EOR!");
        }
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
    }
}