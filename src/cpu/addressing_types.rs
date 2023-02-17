use crate::{cpu::Cpu, memory::Memory};

pub(in crate::cpu) type Addressing = fn(&mut Cpu, &mut Memory);

#[cfg(not(test))] #[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub(in crate::cpu) enum AddressingType {
    Absolute,
    AbsoluteRMW,
    AbsoluteIndirect,
    AbsoluteXIndexed,
    AbsoluteXIndexedRMW,
    AbsoluteXIndexedIndirect,
    AbsoluteYIndexed,
    Accumulator,
    Immediate,
    Implied,
    InterruptSetup,
    InterruptReturn,
    PcRelative,
    StackPull,
    StackPush,
    SubroutineJump,
    SubroutineReturn,
    ZeroPage,
    ZeroPageRMW,
    ZeroPageIndirect,
    ZeroPageIndirectYIndexed,
    ZeroPageXIndexed,
    ZeroPageXIndexedRMW,
    ZeroPageXIndexedIndirect,
    ZeroPageYIndexed,
}

#[cfg(test)] #[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum AddressingType {
    Absolute,
    AbsoluteRMW,
    AbsoluteIndirect,
    AbsoluteXIndexed,
    AbsoluteXIndexedRMW,
    AbsoluteXIndexedIndirect,
    AbsoluteYIndexed,
    Accumulator,
    Immediate,
    Implied,
    InterruptSetup,
    InterruptReturn,
    PcRelative,
    StackPull,
    StackPush,
    SubroutineJump,
    SubroutineReturn,
    ZeroPage,
    ZeroPageRMW,
    ZeroPageIndirect,
    ZeroPageIndirectYIndexed,
    ZeroPageXIndexed,
    ZeroPageXIndexedRMW,
    ZeroPageXIndexedIndirect,
    ZeroPageYIndexed,
}

mod cpu {
    use std::{collections::HashMap};

    use byteorder::{LittleEndian, ByteOrder};

    use crate::{cpu::{addressing_types::Addressing, Cpu, Word, Byte, CpuStatusFlags}, memory::{Memory}};

    use super::AddressingType;
    
    impl Cpu {
        pub(in crate::cpu) fn build_addressing_timing() -> HashMap<AddressingType, u8> {
            return HashMap::from([
                (AddressingType::Absolute, 4),
                (AddressingType::AbsoluteRMW, 6),
                (AddressingType::AbsoluteIndirect, 6),
                (AddressingType::AbsoluteXIndexed, 4),
                (AddressingType::AbsoluteXIndexedRMW, 6),
                (AddressingType::AbsoluteXIndexedIndirect, 6),
                (AddressingType::AbsoluteYIndexed, 4),
                (AddressingType::Accumulator, 2),
                (AddressingType::Immediate, 2),
                (AddressingType::Implied, 2),
                (AddressingType::InterruptSetup, 7),
                (AddressingType::InterruptReturn, 6),
                (AddressingType::PcRelative, 2),
                (AddressingType::StackPull, 4),
                (AddressingType::StackPush, 3),
                (AddressingType::SubroutineJump, 6),
                (AddressingType::SubroutineReturn, 6),
                (AddressingType::ZeroPage, 3),
                (AddressingType::ZeroPageRMW, 5),
                (AddressingType::ZeroPageIndirect, 5),
                (AddressingType::ZeroPageIndirectYIndexed, 5),
                (AddressingType::ZeroPageXIndexed, 4),
                (AddressingType::ZeroPageXIndexedRMW, 6),
                (AddressingType::ZeroPageXIndexedIndirect, 6),
                (AddressingType::ZeroPageYIndexed, 4),

            ]);
        }

        pub(in crate::cpu) fn build_address_type_to_action() -> HashMap<AddressingType, Addressing> {
            return HashMap::from([
                (AddressingType::Absolute, Addressing::from(Cpu::absolute_adressing)),
                (AddressingType::AbsoluteRMW, Addressing::from(Cpu::absolute_adressing)),
                (AddressingType::AbsoluteIndirect, Addressing::from(Cpu::absolute_indirect_addressing)),
                (AddressingType::AbsoluteXIndexed, Addressing::from(Cpu::absolute_x_indexed_adressing)),
                (AddressingType::AbsoluteXIndexedRMW, Addressing::from(Cpu::absolute_x_indexed_adressing)),
                (AddressingType::AbsoluteXIndexedIndirect, Addressing::from(Cpu::absolute_x_indexed_indirect_adressing)),
                (AddressingType::AbsoluteYIndexed, Addressing::from(Cpu::absolute_y_indexed_adressing)),
                (AddressingType::Accumulator, Addressing::from(Cpu::acccumulator_addressing)),
                (AddressingType::Immediate, Addressing::from(Cpu::immediate_addressing)),
                (AddressingType::Implied, Addressing::from(Cpu::implied_addressing)),
                (AddressingType::InterruptSetup, Addressing::from(Cpu::interrupt_setup_addressing)),
                (AddressingType::InterruptReturn, Addressing::from(Cpu::interrupt_return_addressing)),
                (AddressingType::PcRelative, Addressing::from(Cpu::pc_relative_addressing)),
                (AddressingType::StackPull, Addressing::from(Cpu::stack_pull_addressing)),
                (AddressingType::StackPush, Addressing::from(Cpu::stack_push_addressing)),
                (AddressingType::SubroutineJump, Addressing::from(Cpu::absolute_adressing)),
                (AddressingType::SubroutineReturn, Addressing::from(Cpu::implied_addressing)),
                (AddressingType::ZeroPage, Addressing::from(Cpu::zero_page_addressing)),
                (AddressingType::ZeroPageRMW, Addressing::from(Cpu::zero_page_addressing)),
                (AddressingType::ZeroPageIndirect, Addressing::from(Cpu::zero_page_indirect_addressing)),
                (AddressingType::ZeroPageIndirectYIndexed, Addressing::from(Cpu::zero_page_indirect_y_indexed_addressing)),
                (AddressingType::ZeroPageXIndexed, Addressing::from(Cpu::zero_page_x_indexed_addressing)),
                (AddressingType::ZeroPageXIndexedRMW, Addressing::from(Cpu::zero_page_x_indexed_addressing)),
                (AddressingType::ZeroPageXIndexedIndirect, Addressing::from(Cpu::zero_page_x_indexed_indirect_addressing)),
                (AddressingType::ZeroPageYIndexed, Addressing::from(Cpu::zero_page_y_indexed_addressing)),
            ]);
        }

        fn absolute_adressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc + 1)) << 8;
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(memory.read_byte(self.pc - 1));
                    self.pc += 1;
                }
                _ => {}
            }
        }

        // Indirect addressing is only used for JMP so this addressing adjust PC
        fn absolute_indirect_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc + 1)) << 8;
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(memory.read_byte(self.pc - 1));
                    self.pc += 1;
                }
                3 => {
                    self.pc = Word::from(memory.read_byte(self.addressing + 1));
                }
                4 => {
                    self.pc += Word::from(memory.read_byte(self.addressing)) << 8;
                }
                _ => {}
            }
        }

        fn absolute_x_indexed_adressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc + 1)) << 8;
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(memory.read_byte(self.pc - 1)) + Word::from(self.x);
                    self.pc += 1;
                }
                _ => {}
            }
        }

        fn absolute_y_indexed_adressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = Word::from(memory.read_byte(self.pc + 1)) << 8;
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(memory.read_byte(self.pc - 1)) + Word::from(self.y);
                    self.pc += 1;
                }
                _ => {}
            }
        }

        // Indirect addressing is only used for JMP so this addressing adjust PC
        fn absolute_x_indexed_indirect_adressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc + 1)) << 8;
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(memory.read_byte(self.pc - 1));
                    self.pc += 1;
                }
                3 => {
                    self.addressing += Word::from(self.x)
                }
                4 => {
                    self.pc = Word::from(memory.read_byte(self.addressing + 1)) << 8;
                }
                5 => {
                    self.pc += Word::from(memory.read_byte(self.addressing));
                }
                _ => {}
            }
        }
    
        fn acccumulator_addressing(&mut self, _memory: &mut Memory) {}

        fn immediate_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.alu = memory.read_byte(self.pc);
                    self.pc += 1;
                }
                _ => {}
            }
        }

        fn implied_addressing(&mut self, _memory: &mut Memory) {}

        fn interrupt_setup_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                2 => {
                    let mut pc_buff: [Byte; 2] = [0,0];
                    LittleEndian::write_u16(&mut pc_buff, self.pc + 1);
                    self.stack_push(memory, pc_buff[1]);
                }
                3 => {
                    let mut pc_buff: [Byte; 2] = [0,0];
                    LittleEndian::write_u16(&mut pc_buff, self.pc + 1);
                    self.stack_push(memory, pc_buff[0]);
                }
                4 => {
                    self.stack_push(memory, self.ps.bits);
                    self.ps.set(CpuStatusFlags::I, true);
                }
                5 => {
                    self.pc = Word::from(memory.read_byte(0xFFFE));
                }
                6 => {
                    self.pc = Word::from(memory.read_byte(0xFFFF)) << 8;
                }
                _ => {}
            }
        }

        fn interrupt_return_addressing(&mut self, memory: &mut Memory){
            match self.tcu {
                2 => {
                    self.ps = CpuStatusFlags { bits: self.stack_pull(memory) };
                }
                3 => {
                    self.pc = Word::from(self.stack_pull(memory));
                }
                4 => {
                    self.pc += Word::from(self.stack_pull(memory)) << 8;
                }
                _ => {}
            }
        }

        fn pc_relative_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.alu = memory.read_byte(self.pc);
                    self.pc += 1;
                }
                _ => {}
            }
        }

        fn stack_pull_addressing(&mut self, _memory: &mut Memory){}

        fn stack_push_addressing(&mut self, _memory: &mut Memory){}

        fn zero_page_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                _ => {

                }
            }
        }

        fn zero_page_indirect_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                2 => {
                    self.alu = memory.read_byte(self.addressing);
                }
                3 => {
                    self.addressing = Word::from(memory.read_byte(self.addressing + 1)) << 8;
                    self.addressing += Word::from(self.alu);
                }
                _ => {

                }
            }
        }

        fn zero_page_indirect_y_indexed_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                2 => {
                    self.alu = memory.read_byte(self.addressing);
                }
                3 => {
                    self.addressing = Word::from(memory.read_byte(self.addressing + 1)) << 8;
                    self.addressing += Word::from(self.alu) + Word::from(self.y);
                }
                _ => {

                }
            }
        }

        fn zero_page_x_indexed_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(self.x);
                }
                _ => {}
            }
        }

        fn zero_page_x_indexed_indirect_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(self.x);
                }
                3 => {
                    self.alu = memory.read_byte(self.addressing);
                }
                4 => {
                    self.addressing = Word::from(memory.read_byte(self.addressing + 1)) << 8;
                    self.addressing += Word::from(self.alu);
                }
                _ => {

                }
            }
        }
    
        fn zero_page_y_indexed_addressing(&mut self, memory: &mut Memory) {
            match self.tcu {
                1 => {
                    self.addressing = 0x0000;
                    self.addressing = Word::from(memory.read_byte(self.pc));
                    self.pc += 1;
                }
                2 => {
                    self.addressing += Word::from(self.y);
                }
                _ => {

                }
            }
        }
    
        // TODO there are edge cases with extra clocks, neeed to consider those later
    }
}