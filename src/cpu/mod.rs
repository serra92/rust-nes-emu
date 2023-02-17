mod instruction_set;
pub mod instructions;
pub mod sound_waves;

#[cfg(not(test))]
mod addressing_types;

#[cfg(test)]
pub mod addressing_types;

use bitflags::bitflags;
use crate::memory::{Memory};
use std::{collections::{HashMap}, fmt::{self, Display}};
use crate::cpu::instruction_set::Instruction;

use self::{addressing_types::{Addressing, AddressingType}, instruction_set::offset_byte_signed_byte};

// Unsigned Variations
pub type Byte = u8;
pub type Word = u16;

// Signed Variations. Will be used sparsely during instructions whenever needed
pub type SByte = i8;

bitflags! {
    pub struct CpuStatusFlags: Byte {
        const C = 0b0000_0001; // CARRY FLAG
        const Z = 0b0000_0010; // ZERO FLAG
        const I = 0b0000_0100; // INTERRUPT FLAG
        const D = 0b0000_1000; // DECIMAL FLAG
        const B = 0b0001_0000; // BRK FLAG
        // value  0b0010_0000 not in use on 65c02
        const V = 0b0100_0000; // OVERFLOW FLAG
        const N = 0b1000_0000; // NEGATIVE FLAG
    }
}

impl Display for CpuStatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:08b}", self.bits)
    }
}

pub struct Cpu {
    // internal registers
    ir: Byte,
    tcu: Byte,
    alu: Byte,
    addressing: Word,
    reset: bool,

    // Registers
    a: Byte,
    x: Byte,
    y: Byte,
    pc: Word,
    sp: Byte,
    ps: CpuStatusFlags,

    instruction_set: HashMap<Byte, Instruction>,
    addressing_clock_count: HashMap<AddressingType, u8>,
    addressing_action: HashMap<AddressingType, Addressing>,
    instruction_addressing: HashMap<Byte, AddressingType>
}

#[cfg(test)]
pub enum Register {
    A,
    X,
    Y,
    SP
}

impl Cpu {
    // Initialize CPU with given clock cycle (in Hz)
    pub fn build_cpu() -> Cpu {
        return Cpu {

            ir: 0x00,
            tcu: 0x00,
            alu: 0x00,
            addressing: 0x0000,
            reset: false,

            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000,
            sp: 0x00,
            ps: CpuStatusFlags { bits: 0b0010_0000 },
            instruction_set: Cpu::build_instruction_set(),
            addressing_clock_count: Cpu::build_addressing_timing(),
            addressing_action: Cpu::build_address_type_to_action(),
            instruction_addressing: Cpu::build_addressing_type()
        }
    }

    pub fn reset(&mut self, memory: &Memory) {
        self.ps.set(CpuStatusFlags::I, false);
        self.ps.set(CpuStatusFlags::D, false);
        self.ps.set(CpuStatusFlags::B, true);
        self.sp = 0xff;

        self.pc = memory.read_word(0xfffc);

        self.reset = true;
    }

    pub fn exec_cycle(&mut self, memory: &mut Memory) {
        if self.reset {
            self.reset = false;
            self.fetch_instruction(memory);
            return
        }
        self.tcu += 1;
        if let Some(addressing_type) = self.instruction_addressing.get(&self.ir) {
            if let Some(clock_count) = self.addressing_clock_count.get(&addressing_type) {
                if *clock_count == self.tcu {
                    self.fetch_instruction(memory);
                } else {
                    self.run_instruction(memory);
                }
            } else {
                panic!("Improper form for Addressing mode to timing map")
            }
        } else {
            panic!("Invalid instruction arived at CPU clock handling!!!")
        }
    }

    fn fetch_instruction(&mut self, memory: &Memory) {
        self.tcu = 0;
        self.ir = memory.read_byte(self.pc);
        self.pc += 1;
    }

    fn run_instruction(&mut self, memory: &mut Memory) {
        // The timing of the CPU clocks is programmed both on addressing and instruction
        if let Some(addressing) = self.instruction_addressing.get(&self.ir) {
            if let Some(addressing_action) = self.addressing_action.get(addressing) {
                addressing_action(self, memory);
            } else {
                panic!("Improper form on Adressing Mode to action map")
            }
        } else {
            panic!("Invalid instruction arrived at CPU translation!");
        }
        if let Some(instruction) = self.instruction_set.get(&self.ir) {
            instruction(self, memory);
        } else {
            panic!("Invalid instruction arrived at CPU translation!");
        }
    }

    fn stack_push(&mut self, memory: &mut Memory, data: Byte) {
        memory.write_byte(0x0100 + Word::from(self.sp), data);
        self.sp = offset_byte_signed_byte(self.sp, SByte::from(-1) as Byte);
    }
    
    fn stack_pull(&mut self, memory: &mut Memory) -> Byte {
        self.sp = offset_byte_signed_byte(self.sp, 1);
        return memory.read_byte(0x0100 + Word::from(self.sp));
    }

    #[cfg(test)]
    pub fn get_addressing_type_for_opcode(&self, opcode: Byte) -> AddressingType {
        return *self.instruction_addressing.get(&opcode).unwrap();
    }

    #[cfg(test)]
    pub fn get_clocks_for_addressing_type(&self, addr_type: AddressingType) -> u8 {
        return *self.addressing_clock_count.get(&addr_type).unwrap();
    }

    #[cfg(test)]
    pub fn set_register(&mut self, register: Register, data: Byte) {
        match register {
            Register::A => {
                self.a = data
            }
            Register::X => {
                self.x = data
            }
            Register::Y => {
                self.y = data
            }
            Register::SP => {
                self.sp = data
            }
        }
    }

    #[cfg(test)]
    pub fn get_register(&self, register: Register) -> Byte {
        match register {
            Register::A => {
                return self.a;
            }
            Register::X => {
                return self.x;
            }
            Register::Y => {
                return self.y;
            }
            Register::SP => {
                return self.sp;
            }
        }
    }

    #[cfg(test)]
    pub fn get_processor_status(&self) -> CpuStatusFlags {
        return self.ps;
    }

    #[cfg(test)]
    pub fn get_program_counter(&self) -> Word {
        return self.pc;
    }

    #[cfg(test)]
    pub fn is_set(&self, flag: CpuStatusFlags) -> bool {
        return self.ps.contains(flag);
    }

    #[cfg(test)]
    pub fn clear_flag(&mut self, flag: CpuStatusFlags) {
        self.ps.set(flag, false);
    }

    #[cfg(test)]
    pub fn set_flag(&mut self, flag: CpuStatusFlags) {
        self.ps.set(flag, true);
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f, 
            "DEBUG REGISTERS\n\
            \tIR: {:#X}\n\
            \tTCU: {:#X}\n\
            \tALU: {:#X}\n\
            \tADDR: {:#X}\n\
            PROGRAMMING REGISTERS\n\
            \tA: {:#X}\n\
            \tX: {:#X}\n\
            \tY: {:#X}\n\
            \tPC: {:#X}\n\
            \tSP: {:#X}\n\
            \tPS: {}",
            self.ir, 
            self.tcu,
            self.alu,
            self.addressing,
            self.a,
            self.x,
            self.y,
            self.pc,
            self.sp,
            self.ps
        );
    }
}