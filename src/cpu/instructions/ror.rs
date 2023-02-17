use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Zp = 0x66,
    Acc = 0x6A,
    Abs = 0x6E,
    ZpXIdx = 0x76,
    AbsXIdx = 0x7E
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
        (Opcode::Zp.into(), Instruction::from(Cpu::ror)),
        (Opcode::Acc.into(), Instruction::from(Cpu::ror)),
        (Opcode::Abs.into(), Instruction::from(Cpu::ror)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::ror)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::ror)),
    ]);
}

impl crate::cpu::Cpu{
    fn ror(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::AbsoluteRMW | AddressingType::AbsoluteXIndexedRMW | AddressingType::ZeroPageXIndexedRMW => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            let new_carry = self.alu & 0b0000_0001 > 0;
                            self.alu >>= 1;
                            self.alu |= if self.ps.contains(CpuStatusFlags::C) { 0b1000_0000 } else { 0b0000_0000 };
                            self.ps.set(CpuStatusFlags::C, new_carry);
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
                            self.alu = self.a;
                            let new_carry = self.alu & 0b0000_0001 > 0;
                            self.alu >>= 1;
                            self.alu |= if self.ps.contains(CpuStatusFlags::C) { 0b1000_0000 } else { 0b0000_0000 };
                            self.ps.set(CpuStatusFlags::C, new_carry);
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
                            let new_carry = self.alu & 0b0000_0001 > 0;
                            self.alu >>= 1;
                            self.alu |= if self.ps.contains(CpuStatusFlags::C) { 0b1000_0000 } else { 0b0000_0000 };
                            self.ps.set(CpuStatusFlags::C, new_carry);
                        }
                        3 => { /* Emulate RMW extra cycle */ }
                        4 => {
                            memory.write_byte(self.addressing, self.alu);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction ROR!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction ROR!");
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
        self.ps.set(CpuStatusFlags::N, false)
    }
}