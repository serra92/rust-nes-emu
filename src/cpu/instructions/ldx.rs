use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Zp = 0xA6,
    Imm = 0xA2,
    Abs = 0xAE,
    AbsYIdx = 0xBE,
    ZpYIdx = 0xB6,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Zp.into(), AddressingType::ZeroPage),
        (Opcode::Imm.into(), AddressingType::Immediate), 
        (Opcode::Abs.into(), AddressingType::Absolute),
        (Opcode::ZpYIdx.into(), AddressingType::ZeroPageYIndexed),
        (Opcode::AbsYIdx.into(), AddressingType::AbsoluteYIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::ldx)),
        (Opcode::Imm.into(), Instruction::from(Cpu::ldx)),
        (Opcode::Abs.into(), Instruction::from(Cpu::ldx)),
        (Opcode::ZpYIdx.into(), Instruction::from(Cpu::ldx)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::ldx)),
    ]);
}

impl crate::cpu::Cpu{
    fn ldx(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageYIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.x = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.x = self.alu;
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
                            self.x = self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction LDX!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction LDX!");
        }
        self.ps.set(CpuStatusFlags::Z, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::N, self.alu == 0);
    }
}