use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction, offset_byte_signed_byte}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Zp = 0xE6,
    Acc = 0x1A,
    Abs = 0xEE,
    ZpXIdx = 0xF6,
    AbsXIdx = 0xFE
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Zp.into(), AddressingType::ZeroPageRMW),
        (Opcode::Acc.into(), AddressingType::Accumulator),
        (Opcode::Abs.into(), AddressingType::AbsoluteRMW),
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexedRMW),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexedRMW),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::inc)),
        (Opcode::Acc.into(), Instruction::from(Cpu::inc)),
        (Opcode::Abs.into(), Instruction::from(Cpu::inc)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::inc)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::inc)),
    ]);
}

impl crate::cpu::Cpu{
    fn inc(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::AbsoluteRMW | AddressingType::AbsoluteXIndexedRMW | AddressingType::ZeroPageXIndexedRMW => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.alu = offset_byte_signed_byte(self.alu,0x01);
                        }
                        4 => { /* Emulate RMW extra cycle */ }
                        5 => {
                            memory.write_byte(self.addressing, self.alu);
                        }
                        _ => {}
                    }
                }
                AddressingType::Accumulator => {
                    match self.tcu {
                        1 => {
                            self.alu = offset_byte_signed_byte(self.alu,0x01);
                            self.a = self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPageRMW => {
                    match self.tcu {
                        1 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        2 => {
                            self.alu = offset_byte_signed_byte(self.alu,0x01);
                        }
                        3 => { /* Emulate RMW extra cycle */ }
                        4 => {
                            memory.write_byte(self.addressing, self.alu);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction INC!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction INC!");
        }
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
    }
}