use std::collections::HashMap;

use crate::{cpu::{Byte, addressing_types::AddressingType, instruction_set::{Instruction}, Cpu}, memory::Memory};

pub enum Opcode {
    IntSetup = 0x00,
}

impl Into<Byte> for Opcode {
    fn into(self) -> Byte {
        return self as Byte;
    }
}

pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType> {
    return HashMap::from([
        (Opcode::IntSetup.into(), AddressingType::InterruptSetup),
    ])
}

pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
    return HashMap::from([
        (Opcode::IntSetup.into(), Instruction::from(Cpu::brk)),
    ]);
}

impl crate::cpu::Cpu{
    fn brk(&mut self, _memory: &mut Memory) {}
}

#[cfg(test)]
mod tests {
    use crate::{cpu::{Byte, Cpu, instructions::brk, Register, Word}, memory::Memory, test_utils::{TestCpu, TestMemory}};

    const INT_VECTOR: Word = 0xFFFE;
    const START_INTERRUPT: Word = 0xA000;

    #[test]
    fn test_program_interrupt() {
        let mut test_memory = Memory::build_memory();
        // setup vectors
        test_memory.write_word(INT_VECTOR, START_INTERRUPT);
        test_memory.write_word(crate::test_utils::RESET_VECTOR_ADDRESS, crate::test_utils::START_PROGRAM);
        // write program
        test_memory.write_byte(crate::test_utils::START_PROGRAM, brk::Opcode::IntSetup as Byte);
        
        let mut test_cpu = Cpu::build_cpu();
        test_cpu.reset(&test_memory);
        let ps = test_cpu.get_processor_status();
        for _ in 0..7 {
            test_cpu.exec_cycle(&mut test_memory);
        }

        // Make assertions
        let cpu = TestCpu::clone_from_cpu(&test_cpu);
        let memory = TestMemory::clone_from_memory(&test_memory);
        cpu.assert_pc(START_INTERRUPT);
        cpu.assert_register(Register::SP, 0xFF - 3);
        memory.assert_word(0x01FE, crate::test_utils::START_PROGRAM + 2);
        memory.assert_byte(0x01FD, ps.bits());
        
    }
}