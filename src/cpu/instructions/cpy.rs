use std::collections::HashMap;

use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::{Byte, addressing_types::AddressingType, Word, instruction_set::{add, Instruction, test_carry}, Cpu, CpuStatusFlags, SByte}, memory::Memory};

pub enum Opcode {
    Imm = 0xC0,
    Zp = 0xC4,
    Abs = 0xCC,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::Imm.into(), AddressingType::Immediate), 
        (Opcode::Zp.into(), AddressingType::ZeroPage),
        (Opcode::Abs.into(), AddressingType::Absolute),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::Imm.into(), Instruction::from(Cpu::cpy)),
        (Opcode::Zp.into(), Instruction::from(Cpu::cpy)),
        (Opcode::Abs.into(), Instruction::from(Cpu::cpy)),
    ]);
}

impl crate::cpu::Cpu{
    fn cpy(&mut self, memory: &mut Memory) {
        let mut result_data: Option<Word> = None;
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            match addressing_type {
                AddressingType::Absolute => {
                    match self.tcu {
                        2 => {
                            self.alu = memory.read_byte(self.addressing);
                        }
                        3 => {
                            let mut result_buff: [Byte; 2] = [0,0];
                            // Note: On 2-complement, A-B = A + NOT(B) + 1
                            let sub = add(self.y, !self.alu, true);
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
                            let sub = add(self.y, !self.alu, true);
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
                            let sub = add(self.y, !self.alu, true);
                            LittleEndian::write_u16(&mut result_buff, sub);
                            result_data = Some(sub);
                            self.alu = result_buff[0];
                        }
                        _ => {}
                    }
                }
                _ => {
                    panic!("Missing addressing type for instruction CPY!");
                }
            }
        } else {
            panic!("Missing addressing type for instruction CPY!");
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
    use crate::{test_utils::{TestCpu, setup_test}, cpu::{Byte, CpuStatusFlags, Register, Word}};
    use crate::cpu::instructions::cpy;

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
                None,
                None,
                Some(0x01),
                None,
                cpy::Opcode::Abs as Byte,
                0x01
            );
        cpu.assert_register(Register::Y, 0x01);
        memory.assert_byte(crate::test_utils::ABS_PTR, 0x01);
    }

    #[test]
    fn test_imm() {
        let (cpu, _) =
            setup_test(
                None,
                None,
                Some(0x01),
                None,
                cpy::Opcode::Imm as Byte,
                0x01
            );
        cpu.assert_register(Register::Y, 0x01);
    }

    #[test]
    fn test_zp() {
        let (cpu, memory) =
            setup_test(
                None,
                None,
                Some(0x01),
                None,
                cpy::Opcode::Zp as Byte,
                0x01
            );
        cpu.assert_register(Register::Y, 0x01);
        memory.assert_byte(Word::from(crate::test_utils::ZP_PTR), 0x01);
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
        for (y_reg, data, c_flag, z_flag, n_flag) in test_cases {
            let (cpu, _) =
                setup_test(
                    None,
                    None,
                    Some(y_reg),
                    None,
                    cpy::Opcode::Imm as Byte,
                    data
                );
            test_flags(cpu, c_flag, z_flag, n_flag);
        }
    }
}