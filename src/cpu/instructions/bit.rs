use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags}, memory::Memory};

pub enum Opcode {
    Zp = 0x24,
    Abs = 0x2C,
    ZpXIdx = 0x34,
    AbsXIdx = 0x3C,
    Imm = 0x89,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Zp.into(), AddressingType::ZeroPage),
        (Opcode::Abs.into(), AddressingType::Absolute),
        (Opcode::ZpXIdx.into(), AddressingType::ZeroPageXIndexed),
        (Opcode::AbsXIdx.into(), AddressingType::AbsoluteXIndexed),
        (Opcode::Imm.into(), AddressingType::Immediate),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Zp.into(), Instruction::from(Cpu::bit)),
        (Opcode::Abs.into(), Instruction::from(Cpu::bit)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::bit)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::bit)),
        (Opcode::Imm.into(), Instruction::from(Cpu::bit)),
    ]);
}

impl crate::cpu::Cpu{
    // Flags N and V need to be set mid instruction to ensure they capture original status from memory.
    // On immediate addressing, only Z flag is set
    fn bit(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute | AddressingType::AbsoluteXIndexed | AddressingType::ZeroPageXIndexed => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                            self.ps.set(CpuStatusFlags::N, self.alu & 0b1000_0000 > 0);
                            self.ps.set(CpuStatusFlags::V, self.alu & 0b0100_0000 > 0);
                        }
                        3 => {
                            self.alu &= self.a;
                        }
                        _ => {}
                    }
                }
                AddressingType::Immediate => {
                    match self.tcu {
                        1 => {
                            self.alu &= self.a;
                        }
                        _ => {}
                    }
                }
                AddressingType::ZeroPage => {
                    match self.tcu {
                        1 => {
                            self.alu = memory.read_byte(self.addressing);
                            self.ps.set(CpuStatusFlags::N, self.alu & 0b1000_0000 > 0);
                            self.ps.set(CpuStatusFlags::V, self.alu & 0b0100_0000 > 0);
                        }
                        2 => {
                            self.alu &= self.a;
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction BIT!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction BIT!");
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
    }
}

#[cfg(test)]
mod tests {
    const PTR_OFFSET: Byte = 0x02;

    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register, Word}};
    use crate::cpu::instructions::bit;

    fn test_flags(cpu: TestCpu, z_flag: bool, v_flag:bool, n_flag: bool) {
        cpu.assert_status(CpuStatusFlags::Z, z_flag);
        cpu.assert_status(CpuStatusFlags::V, v_flag);
        cpu.assert_status(CpuStatusFlags::N, n_flag);
    }

    // Test each addressing that does not set flags
    #[test]
    fn test_abs() {
        let (cpu, memory) =
            setup_test(
                Some(0xFF),
                None,
                None,
                None,
                bit::Opcode::Abs as Byte,
                0xAA
            );
        cpu.assert_register(Register::A, 0xFF);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0xAA);
    }

    #[test]
    fn test_abs_x_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0xFF),
                Some(PTR_OFFSET),
                None,
                None,
                bit::Opcode::AbsXIdx as Byte,
                0xAA
            );
        cpu.assert_register(Register::A, 0xFF);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0xAA);
    }

    #[test]
    fn test_imm() {
        let (cpu, _) =
            setup_test(
                Some(0xFF),
                None,
                None,
                None,
                bit::Opcode::Imm as Byte,
                0x01
            );
        cpu.assert_register(Register::A, 0xFF);
    }

    #[test]
    fn test_zp() {
        let (cpu, memory) =
            setup_test(
                Some(0xFF),
                None,
                None,
                None,
                bit::Opcode::Zp as Byte,
                0xAA
            );
        cpu.assert_register(Register::A, 0xFF);
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0xAA);
    }

    #[test]
    fn test_zp_x_idx() {
        let (cpu, memory) =
            setup_test(
                Some(0xFF),
                Some(PTR_OFFSET),
                None,
                None,
                bit::Opcode::ZpXIdx as Byte,
                0xAA
            );
        cpu.assert_register(Register::A, 0xFF);
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0xAA);
    }

    // Running unit tests for flags and operation 
    #[test]
    fn test_op_flags() {
        let test_cases: Vec<(Byte, Byte, bool, bool, bool)> = vec![
            (0xFF, 0x00, true, false, false),
            (0x00, 0xFF, true, true, true),
            (0x80, 0xFF, false, true, true),
            (0xC0, 0x80, false, false, true),
            (0xC0, 0x40, false, true, false),
            (0xFF, 0x20, false, false, false)
        ];
        for (a_reg, data, z_flag, v_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    Some(a_reg),
                    None,
                    None,
                    None,
                    bit::Opcode::Abs as Byte,
                    data
                );

            println!("0x{:08b} & 0x{:08b} = 0b{:08b}", a_reg, data, a_reg & data);
            
            test_flags(cpu, z_flag, v_flag, n_flag);
        }
    }
}