use std::collections::HashMap;

use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::{Byte, addressing_types::AddressingType, Word, instruction_set::{add, Instruction, test_carry}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0xC1,
    Zp = 0xC5,
    Imm = 0xC9,
    Abs = 0xCD,
    ZpIndYIdx = 0xD1,
    ZpInd = 0xD2,
    ZpXIdx = 0xD5,
    AbsYIdx = 0xD9,
    AbsXIdx = 0xDD
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::cmp)),
        (Opcode::Zp.into(), Instruction::from(Cpu::cmp)),
        (Opcode::Imm.into(), Instruction::from(Cpu::cmp)),
        (Opcode::Abs.into(), Instruction::from(Cpu::cmp)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::cmp)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::cmp)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::cmp)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::cmp)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::cmp)),
    ]);
}

impl crate::cpu::Cpu{
    fn cmp(&mut self, memory: &mut Memory) {
        let mut result_data: Option<Word> = None;
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.a, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.a, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
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
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.a, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
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
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.a, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
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
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.a, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction CMP!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction CMP!");
        }
        if let Some(data) = result_data {
            self.ps.set(CpuStatusFlags::C,test_carry(data));
            self.ps.set(CpuStatusFlags::Z, self.alu == 0);
            self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0);   
        }
    }
}

#[cfg(test)]
mod tests {
    const PTR_OFFSET: Byte = 0x02;

    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register, Word}};
    use crate::cpu::instructions::cmp;

    fn test_flags(cpu: TestCpu, c_flag: bool, z_flag: bool, n_flag: bool) {
        cpu.assert_status(CpuStatusFlags::C, c_flag);
        cpu.assert_status(CpuStatusFlags::Z, z_flag);
        cpu.assert_status(CpuStatusFlags::N, n_flag);
    }

    // Test each addressing with basic sum that does not set flags
    #[test]
    fn test_abs() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                cmp::Opcode::Abs as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x01);
    }

    #[test]
    fn test_abs_x_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                cmp::Opcode::AbsXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x01);
    }

    #[test]
    fn test_abs_y_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                cmp::Opcode::AbsYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x01);
    }

    #[test]
    fn test_imm() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                cmp::Opcode::Imm as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                cmp::Opcode::Zp as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0x01);
    }

    #[test]
    fn test_zp_x_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                cmp::Opcode::ZpXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0x01);
    }

    #[test]
    fn test_zp_ind() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                cmp::Opcode::ZpInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::IND_PTR, 0x01);
    }

    #[test]
    fn test_zp_x_idx_ind() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                cmp::Opcode::ZpXIdxInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::IND_PTR, 0x01);
    }

    #[test]
    fn test_zp_ind_y_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                cmp::Opcode::ZpIndYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
        memory.assert_byte(crate::test_utils::IND_PTR, 0x01);
    }

    #[test]
    fn test_op_flags() {
        let test_cases: Vec<(Byte, Byte, bool, bool, bool)> = vec![
            (0x50, 0xf0, false, false, false),
            (0x50, 0xb0, false, false, true),
            (0x50, 0x30, true, false, false),
            (0xd0, 0x30, true, false, true),
            (0x03, 0x03, true, true, false)
        ];
        for (a_reg, data, c_flag, z_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    Some(a_reg),
                    None,
                    None,
                    None,
                    cmp::Opcode::Imm as Byte,
                    data
                );
            test_flags(cpu, c_flag, z_flag, n_flag);
        }
    }
}