use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0x01,
    Zp = 0x05,
    Imm = 0x09,
    Abs = 0x0D,
    ZpIndYIdx = 0x11,
    ZpInd = 0x12,
    ZpXIdx = 0x15,
    AbsYIdx = 0x19,
    AbsXIdx = 0x1D
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::ora)),
        (Opcode::Zp.into(), Instruction::from(Cpu::ora)),
        (Opcode::Imm.into(), Instruction::from(Cpu::ora)),
        (Opcode::Abs.into(), Instruction::from(Cpu::ora)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::ora)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::ora)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::ora)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::ora)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::ora)),
    ]);
}

impl crate::cpu::Cpu{
    fn ora(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.alu |= self.a;
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.alu |= self.a;
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
                            self.alu |= self.a;
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
                            self.alu |= self.a;
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
                            self.alu |= self.a;
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction ORA!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction ORA!");
        }
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
    }
}