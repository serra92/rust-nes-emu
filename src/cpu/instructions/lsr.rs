use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Zp = 0x46,
    Acc = 0x4A,
    Abs = 0x4E,
    ZpXIdx = 0x56,
    AbsXIdx = 0x5E
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
        (Opcode::Zp.into(), Instruction::from(Cpu::lsr)),
        (Opcode::Acc.into(), Instruction::from(Cpu::lsr)),
        (Opcode::Abs.into(), Instruction::from(Cpu::lsr)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::lsr)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::lsr)),
    ]);
}

impl crate::cpu::Cpu{
    fn lsr(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::AbsoluteRMW | AddressingType::AbsoluteXIndexedRMW | AddressingType::ZeroPageXIndexedRMW => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.ps.set(CpuStatusFlags::C, self.alu & 0b0000_0001 > 0);
                            self.alu >>= 1;
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
                            self.ps.set(CpuStatusFlags::C, self.a & 0b0000_0001 > 0);
                            self.a >>= 1;
                            self.alu = self.a;
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
                            self.ps.set(CpuStatusFlags::C, self.alu & 0b0000_0001 > 0);
                            self.alu >>= 1;
                        }
                        3 => { /* Emulate RMW extra cycle */ }
                        4 => {
                            memory.write_byte(self.addressing, self.alu);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction LSR!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction LSR!");
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
        self.ps.set(CpuStatusFlags::N, false)
    }
}