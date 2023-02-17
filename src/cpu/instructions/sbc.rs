use std::collections::HashMap;

use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::{Byte, addressing_types::AddressingType, Word, instruction_set::{add, Instruction, test_carry, test_overflow}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0xE1,
    Zp = 0xE5,
    Imm = 0xE9,
    Abs = 0xED,
    ZpIndYIdx = 0xF1,
    ZpInd = 0xF2,
    ZpXIdx = 0xF5,
    AbsYIdx = 0xF9,
    AbsXIdx = 0xFD
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::sbc)),
        (Opcode::Zp.into(), Instruction::from(Cpu::sbc)),
        (Opcode::Imm.into(), Instruction::from(Cpu::sbc)),
        (Opcode::Abs.into(), Instruction::from(Cpu::sbc)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::sbc)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::sbc)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::sbc)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::sbc)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::sbc)),
    ]);
}

impl crate::cpu::Cpu{
    // Details on why it works using addition with one's complement here: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    fn sbc(&mut self, memory: &mut Memory) {
        let mut result_data: Option<Word> = None;
        let orig_data = self.a;
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            let data: Word = add(self.a, !self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            let data: Word = add(self.a, !self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
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
                            let data: Word = add(self.a, !self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
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
                            let data: Word = add(self.a, !self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
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
                            let data: Word = add(self.a, !self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction SBC!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction SBC!");
        }
        if let Some(data) = result_data {
            self.ps.set(CpuStatusFlags::C,test_carry(data));
            self.ps.set(CpuStatusFlags::Z, self.a == 0);
            self.ps.set(CpuStatusFlags::V, test_overflow(orig_data, self.alu, self.a));
            self.ps.set(CpuStatusFlags::N, (self.a as SByte) < 0);   
        }
    }
}