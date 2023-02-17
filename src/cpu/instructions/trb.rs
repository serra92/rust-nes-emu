use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Abs = 0x1C,
    Zp = 0x14,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Abs.into(), AddressingType::AbsoluteRMW),
        (Opcode::Zp.into(), AddressingType::ZeroPageRMW),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Abs.into(), Instruction::from(Cpu::trb)),
        (Opcode::Zp.into(), Instruction::from(Cpu::trb)),
    ]);
}

impl crate::cpu::Cpu{
    fn trb(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::AbsoluteRMW => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.alu &= !self.a;
                        }
                        4 => {
                            memory.write_byte(self.addressing, self.alu)
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
                            self.alu &= !self.a;
                        }
                        3 => {
                            memory.write_byte(self.addressing, self.alu)
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction TRB!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction TRB!");
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
    }
}