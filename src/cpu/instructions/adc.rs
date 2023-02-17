use std::collections::HashMap;

use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::{Byte, addressing_types::AddressingType, Word, instruction_set::{add, Instruction, test_carry, test_overflow}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

#[derive(Copy, Clone)]
pub enum Opcode {
    ZpXIdxInd = 0x61,
    Zp = 0x65,
    Imm = 0x69,
    Abs = 0x6D,
    ZpIndYIdx = 0x71,
    ZpInd = 0x72,
    ZpXIdx = 0x75,
    AbsYIdx = 0x79,
    AbsXIdx = 0x7D
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::adc)),
        (Opcode::Zp.into(), Instruction::from(Cpu::adc)),
        (Opcode::Imm.into(), Instruction::from(Cpu::adc)),
        (Opcode::Abs.into(), Instruction::from(Cpu::adc)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::adc)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::adc)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::adc)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::adc)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::adc)),
    ]);
}

impl crate::cpu::Cpu{
    fn adc(&mut self, memory: &mut Memory) {
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
                            let data: Word = add(self.a, self.alu, self.ps.contains(CpuStatusFlags::C));
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
                            let data: Word = add(self.a, self.alu, self.ps.contains(CpuStatusFlags::C));
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
                            let data: Word = add(self.a, self.alu, self.ps.contains(CpuStatusFlags::C));
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
                            let data: Word = add(self.a, self.alu, self.ps.contains(CpuStatusFlags::C));
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
                            let data: Word = add(self.a, self.alu, self.ps.contains(CpuStatusFlags::C));
                            result_data = Some(data);
                            let mut data_arr:[Byte; 2] = [0,0];
                            LittleEndian::write_u16(&mut data_arr,data);
                            self.a = data_arr[0];
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction ADC!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction ADC!");
        }
        if let Some(data) = result_data {
            self.ps.set(CpuStatusFlags::C,test_carry(data));
            self.ps.set(CpuStatusFlags::Z, self.a == 0);
            self.ps.set(CpuStatusFlags::V, test_overflow(orig_data, self.alu, self.a));
            self.ps.set(CpuStatusFlags::N, (self.a as SByte) < 0);   
        }
    }
}

#[cfg(test)]
mod tests {
    const PTR_OFFSET: Byte = 0x02;

    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register}};
    use crate::cpu::instructions::adc;

    fn test_flags(cpu: TestCpu, c_flag: bool, z_flag: bool, v_flag: bool, n_flag: bool) {
        cpu.assert_status(CpuStatusFlags::C, c_flag);
        cpu.assert_status(CpuStatusFlags::Z, z_flag);
        cpu.assert_status(CpuStatusFlags::V, v_flag);
        cpu.assert_status(CpuStatusFlags::N, n_flag);
    }

    // Test each addressing with basic sum that does not set flags
    #[test]
    fn test_abs() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                adc::Opcode::Abs as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_abs_x_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                adc::Opcode::AbsXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_abs_y_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                adc::Opcode::AbsYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_imm() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                adc::Opcode::Imm as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_zp() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                adc::Opcode::Zp as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_zp_x_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                adc::Opcode::ZpXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_zp_ind() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                adc::Opcode::ZpInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_zp_x_idx_ind() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                adc::Opcode::ZpXIdxInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_zp_ind_y_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                adc::Opcode::ZpIndYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x02);
    }

    // Running unit tests for flags, based on following link: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    // Added extra cases for testing the zero flag 
    #[test]
    fn test_op_flags() {
        let test_cases: Vec<(Byte, Byte, Byte, bool, bool, bool, bool)> = vec![
            (0x50, 0x10, 0x60, false, false, false, false),
            (0x50, 0x50, 0xa0, false, false, true, true),
            (0x50, 0x90, 0xe0, false, false, false, true),
            (0x50, 0xd0, 0x20, true, false, false, false),
            (0xd0, 0x10, 0xe0, false, false, false, true),
            (0xd0, 0x50, 0x20, true, false, false, false),
            (0xd0, 0x90, 0x60, true, false, true, false),
            (0xd0, 0xd0, 0xa0, true, false, false, true),
            (0x00, 0x00, 0x00, false, true, false, false),
            (0xFF, 0x01, 0x00, true, true, false, false)
        ];
        for (a_reg, data, result, c_flag, z_flag, v_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    Some(a_reg),
                    None,
                    None,
                    None,
                    adc::Opcode::Imm as Byte,
                    data
                );
            cpu.assert_register(Register::A, result);
            test_flags(cpu, c_flag, z_flag, v_flag, n_flag);
        }
    }
}