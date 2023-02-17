use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    ZpXIdxInd = 0x21,
    Zp = 0x25,
    Imm = 0x29,
    Abs = 0x2D,
    ZpIndYIdx = 0x31,
    ZpInd = 0x32,
    ZpXIdx = 0x35,
    AbsYIdx = 0x39,
    AbsXIdx = 0x3D
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
        (Opcode::ZpXIdxInd.into(), Instruction::from(Cpu::and)),
        (Opcode::Zp.into(), Instruction::from(Cpu::and)),
        (Opcode::Imm.into(), Instruction::from(Cpu::and)),
        (Opcode::Abs.into(), Instruction::from(Cpu::and)),
        (Opcode::ZpIndYIdx.into(), Instruction::from(Cpu::and)),
        (Opcode::ZpInd.into(), Instruction::from(Cpu::and)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::and)),
        (Opcode::AbsYIdx.into(), Instruction::from(Cpu::and)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::and)),
    ]);
}

impl crate::cpu::Cpu{
    fn and(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                // ALU contains address to data
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::AbsoluteYIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.a &= self.alu;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.a &= self.alu;
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
                            self.a &= self.alu;
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
                            self.a &= self.alu;
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
                            self.a &= self.alu;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction AND!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction AND!");
        }
        self.ps.set(CpuStatusFlags::Z, self.a == 0);
        self.ps.set(CpuStatusFlags::N, (self.a as SByte) < 0);
    }
}

#[cfg(test)]
mod tests {
    const PTR_OFFSET: Byte = 0x02;

    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register}};
    use crate::cpu::instructions::and;

    fn test_flags(cpu: TestCpu, z_flag: bool, n_flag: bool) {
        cpu.assert_status(CpuStatusFlags::Z, z_flag);
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
                and::Opcode::Abs as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_abs_x_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                and::Opcode::AbsXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_abs_y_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                and::Opcode::AbsYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_imm() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                and::Opcode::Imm as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                and::Opcode::Zp as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp_x_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                and::Opcode::ZpXIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp_ind() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                and::Opcode::ZpInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp_x_idx_ind() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                Some(PTR_OFFSET),
                None,
                None,
                and::Opcode::ZpXIdxInd as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    #[test]
    fn test_zp_ind_y_idx() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                Some(PTR_OFFSET),
                None,
                and::Opcode::ZpIndYIdx as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0x01);
    }

    // Running unit tests for flags and operation 
    #[test]
    fn test_op_flags() {
        let test_cases: Vec<(Byte, Byte, Byte, bool, bool)> = vec![
            (0xFF, 0xAA, 0xAA, false, true),
            (0xCC, 0x66, 0x44, false, false),
            (0xFE, 0x01, 0x00, true, false)
        ];
        for (a_reg, data, result, z_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    Some(a_reg),
                    None,
                    None,
                    None,
                    and::Opcode::Imm as Byte,
                    data
                );
            cpu.assert_register(Register::A, result);
            test_flags(cpu, z_flag, n_flag);
        }
    }
}