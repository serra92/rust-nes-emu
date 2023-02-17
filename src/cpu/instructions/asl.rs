use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Zp = 0x06,
    Acc = 0x0A,
    Abs = 0x0E,
    ZpXIdx = 0x16,
    AbsXIdx = 0x1E
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
        (Opcode::Zp.into(), Instruction::from(Cpu::asl)),
        (Opcode::Acc.into(), Instruction::from(Cpu::asl)),
        (Opcode::Abs.into(), Instruction::from(Cpu::asl)),
        (Opcode::ZpXIdx.into(), Instruction::from(Cpu::asl)),
        (Opcode::AbsXIdx.into(), Instruction::from(Cpu::asl)),
    ]);
}

impl crate::cpu::Cpu{
    fn asl(&mut self, memory: &mut Memory) {
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::AbsoluteRMW | AddressingType::AbsoluteXIndexedRMW | AddressingType::ZeroPageXIndexedRMW => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            self.ps.set(CpuStatusFlags::C, self.alu & 0b1000_0000 > 0);
                            self.alu <<= 1;
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
                            self.ps.set(CpuStatusFlags::C, self.a & 0b1000_0000 > 0);
                            self.a <<= 1;
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
                            self.ps.set(CpuStatusFlags::C, self.alu & 0b1000_0000 > 0);
                            self.alu <<= 1;
                        }
                        3 => { /* Emulate RMW extra cycle */ }
                        4 => {
                            memory.write_byte(self.addressing, self.alu);
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction ASL!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction ASL!");
        }
        self.ps.set(CpuStatusFlags::Z, self.alu == 0);
        self.ps.set(CpuStatusFlags::N, (self.alu as SByte) < 0)
    }
}

#[cfg(test)]
mod tests {
    const PTR_OFFSET: Byte = 0x02;

    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register, Word}};
    use crate::cpu::instructions::asl;

    fn test_flags(cpu: TestCpu, c_flag: bool, z_flag: bool, n_flag: bool) {
        cpu.assert_status(CpuStatusFlags::C, c_flag);
        cpu.assert_status(CpuStatusFlags::Z, z_flag);
        cpu.assert_status(CpuStatusFlags::N, n_flag);
    }

    // Test each addressing that does not set flags
    #[test]
    fn test_acc() {
        let (cpu, _) =
            setup_test(
                Some(0x01),
                None,
                None,
                None,
                asl::Opcode::Acc as Byte,
                0x00
            );
        cpu.assert_register(Register::A, 0x02);
    }

    #[test]
    fn test_abs() {
        let (_, memory) =
            setup_test(
                None,
                None,
                None,
                None,
                asl::Opcode::Abs as Byte,
                0x01
            );
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x02);
    }

    #[test]
    fn test_abs_x_idx() {
        let (_, memory) =
            setup_test(
                None,
                Some(PTR_OFFSET),
                None,
                None,
                asl::Opcode::AbsXIdx as Byte,
                0x01
            );
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x02);
    }

    #[test]
    fn test_zp() {
        let (_, memory) =
            setup_test(
                None,
                None,
                None,
                None,
                asl::Opcode::Zp as Byte,
                0x01
            );
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0x02);
    }

    #[test]
    fn test_zp_x_idx() {
        let (_, memory) =
            setup_test(
                None,
                Some(PTR_OFFSET),
                None,
                None,
                asl::Opcode::ZpXIdx as Byte,
                0x01
            );
            memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0x02);
    }

    // Running unit tests for flags and operation 
    #[test]
    fn test_op_flags() {
        let test_cases: Vec<(Byte, Byte, bool, bool, bool)> = vec![
            (0x01, 0x02, false, false, false),
            (0x00, 0x00, false, true, false),
            (0xC2, 0x84, true, false, true),
            (0x42, 0x84, false, false, true),
            (0x80, 0x00, true, true, false)
        ];
        for (a_reg, result, c_flag, z_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    Some(a_reg),
                    None,
                    None,
                    None,
                    asl::Opcode::Acc as Byte,
                    0x00
                );
            cpu.assert_register(Register::A, result);
            test_flags(cpu, c_flag, z_flag, n_flag);
        }
    }
}