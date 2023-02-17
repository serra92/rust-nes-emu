use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Zp = 0xA6,
    Imm = 0xA2,
    Abs = 0xAE,
    AbsXIdx = 0xBE,
    ZpXIdx = 0xB6,
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
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexed),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexed),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::ldy)),
        (Opcode::Imm.into(), Instruction::from(Cpu::ldy)),
        (Opcode::Abs.into(), Instruction::from(Cpu::ldy)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::ldy)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::ldy)),
    ]);
}

impl crate::cpu::Cpu{
    fn ldy(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.y = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.y = self.alu;
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
                            self.y = self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction LDY!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction LDY!");
        }
        self.ps.set(CpuStatusFlags::Z, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::N, self.alu == 0);
    }
}