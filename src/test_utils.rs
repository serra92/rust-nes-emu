#[cfg(test)]
pub const RESET_VECTOR_ADDRESS: crate::cpu::Word = 0xFFFC;
#[cfg(test)]
pub const START_PROGRAM: crate::cpu::Word = 0x8000;
#[cfg(test)]
pub const IND_PTR: crate::cpu::Word = 0x7000;
#[cfg(test)]
pub const ABS_PTR: crate::cpu::Word = 0x6000;
#[cfg(test)]
pub const ZP_PTR: crate::cpu::Byte = 0x20;

#[cfg(test)] #[derive(Copy, Clone)]
pub struct TestMemory {
    memory: crate::memory::Memory,
    next_address: crate::cpu::Word,
}

#[cfg(test)]
pub enum OperandType {
    None,
    Byte,
    Word
}

#[cfg(test)]
pub struct TestCpu {
    // Registers
    a: crate::cpu::Byte,
    x: crate::cpu::Byte,
    y: crate::cpu::Byte,
    pc: crate::cpu::Word,
    sp: crate::cpu::Byte,
    ps: crate::cpu::CpuStatusFlags
}

#[cfg(test)]
impl TestCpu {
    pub fn clone_from_cpu(cpu: &crate::cpu::Cpu) -> TestCpu {
        return TestCpu {
            a: cpu.get_register(crate::cpu::Register::A),
            x: cpu.get_register(crate::cpu::Register::X),
            y: cpu.get_register(crate::cpu::Register::Y),
            sp: cpu.get_register(crate::cpu::Register::SP),
            ps: cpu.get_processor_status(),
            pc: cpu.get_program_counter(),
        }
    }

    pub fn assert_register(&self, register: crate::cpu::Register, data: crate::cpu::Byte) {
        let register_content: crate::cpu::Byte = match register {
            crate::cpu::Register::A => self.a,
            crate::cpu::Register::X => self.x,
            crate::cpu::Register::Y => self.y,
            crate::cpu::Register::SP => self.sp
        };
        assert_eq!(register_content, data);
    }

    pub fn assert_status(&self, status_flags: crate::cpu::CpuStatusFlags, value: bool) {
        assert_eq!(self.ps.contains(status_flags), value);
    }

    pub fn assert_pc(&self, address: crate::cpu::Word){
        assert_eq!(self.pc, address);
    }
}

#[cfg(test)]
impl TestMemory {
    pub fn clone_from_memory(memory: &crate::memory::Memory) -> TestMemory {
        return TestMemory { memory: memory.clone(), next_address: 0x0000 }
    }

    pub fn build_test_memory(res_vec: crate::cpu::Word) -> TestMemory {
        let mut memory: crate::memory::Memory = crate::memory::Memory::build_memory();
        memory.write_word(RESET_VECTOR_ADDRESS, res_vec);
        return TestMemory {
            memory,
            next_address: res_vec
        }
    }

    pub fn write_instruction(&mut self, opcode: crate::cpu::Byte, operand: OperandType, data: crate::cpu::Word) {
        self.memory.write_byte(self.next_address, opcode);
        self.next_address += 1;
        match operand {
            OperandType::Byte => {
                self.memory.write_byte(self.next_address, data as crate::cpu::Byte);
                self.next_address += 1;
            }
            OperandType::Word => {
                self.memory.write_word(self.next_address, data);
                self.next_address += 2;
            }
            _ => {}
        }
    }

    pub fn get_memory(&self) -> crate::memory::Memory {
        return self.memory
    }

    pub fn write_data_byte(&mut self, address: crate::cpu::Word, data: crate::cpu::Byte) {
        self.memory.write_byte(address, data);
    }

    pub fn write_data_word(&mut self, address: crate::cpu::Word, data: crate::cpu::Word) {
        self.memory.write_word(address, data);
    }

    pub fn assert_byte(&self, address: crate::cpu::Word, data: crate::cpu::Byte) {
        assert_eq!(self.memory.read_byte(address), data);
    }

    pub fn assert_word(&self, address: crate::cpu::Word, data: crate::cpu::Word) {
        assert_eq!(self.memory.read_word(address), data);
    }
}

#[cfg(test)]
fn setup_cpu(registers: Vec<(crate::cpu::Register, crate::cpu::Byte)>) -> crate::cpu::Cpu {
    let mut test_cpu = crate::cpu::Cpu::build_cpu();
    for (register, data) in registers {
        test_cpu.set_register(register, data);
    }
    return test_cpu;
}

#[cfg(test)]
fn setup_memory(reset_vector: crate::cpu::Word, program: Vec<(crate::cpu::Byte, OperandType, crate::cpu::Word)>, program_data: Vec<(crate::cpu::Word, crate::cpu::Byte)>) -> TestMemory {
    let mut test_memory: TestMemory = TestMemory::build_test_memory(reset_vector);
    for (opcode, op_type, data) in program {
        test_memory.write_instruction(opcode, op_type, data)
    }
    for (address, data) in program_data {
        test_memory.write_data_byte(address, data);
    }
    return test_memory;
}

#[cfg(test)]
pub fn setup_test(a_reg: Option<crate::cpu::Byte>, x_reg: Option<crate::cpu::Byte>, y_reg: Option<crate::cpu::Byte>, sp_reg: Option<crate::cpu::Byte>, opcode: crate::cpu::Byte, data: crate::cpu::Byte) -> (TestCpu, TestMemory) {
    return setup_test_with_ps(a_reg, x_reg, y_reg, sp_reg, crate::cpu::CpuStatusFlags::empty(), opcode, data)
}

#[cfg(test)]
pub fn setup_test_with_ps(a_reg: Option<crate::cpu::Byte>, x_reg: Option<crate::cpu::Byte>, y_reg: Option<crate::cpu::Byte>, sp_reg: Option<crate::cpu::Byte>, ps_flags: crate::cpu::CpuStatusFlags, opcode: crate::cpu::Byte, data: crate::cpu::Byte) -> (TestCpu, TestMemory) {
    let mut registers: Vec<(crate::cpu::Register, crate::cpu::Byte)> = Vec::new();
    if let Some(reg) = a_reg {
        registers.push((crate::cpu::Register::A, reg))
    }
    if let Some(reg) = x_reg {
        registers.push((crate::cpu::Register::X, reg))
    }
    if let Some(reg) = y_reg {
        registers.push((crate::cpu::Register::Y, reg))
    }
    if let Some(reg) = sp_reg {
        registers.push((crate::cpu::Register::SP, reg))
    }
    // setup CPU
    let mut test_cpu: crate::cpu::Cpu = setup_cpu(registers);
    test_cpu.set_flag(ps_flags);
    let mut program: Vec<(crate::cpu::Byte, OperandType, crate::cpu::Word)> = Vec::new();
    let addressing_type: crate::cpu::addressing_types::AddressingType = test_cpu.get_addressing_type_for_opcode(opcode);
    match addressing_type {
        crate::cpu::addressing_types::AddressingType::Absolute |
        crate::cpu::addressing_types::AddressingType::AbsoluteRMW |
        crate::cpu::addressing_types::AddressingType::AbsoluteIndirect |
        crate::cpu::addressing_types::AddressingType::SubroutineJump => {
            program.push((opcode, OperandType::Word, ABS_PTR));
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexed |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedRMW |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedIndirect => {
            program.push((opcode, OperandType::Word, ABS_PTR - crate::cpu::Word::from(test_cpu.get_register(crate::cpu::Register::X))));
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteYIndexed => {
            program.push((opcode, OperandType::Word, ABS_PTR - crate::cpu::Word::from(test_cpu.get_register(crate::cpu::Register::Y))));
        }
        crate::cpu::addressing_types::AddressingType::Immediate |
        crate::cpu::addressing_types::AddressingType::PcRelative => {
            program.push((opcode, OperandType::Byte, crate::cpu::Word::from(data)));
        }
        crate::cpu::addressing_types::AddressingType::Accumulator |
        crate::cpu::addressing_types::AddressingType::Implied |
        crate::cpu::addressing_types::AddressingType::StackPull |
        crate::cpu::addressing_types::AddressingType::StackPush |
        crate::cpu::addressing_types::AddressingType::InterruptReturn |
        crate::cpu::addressing_types::AddressingType::SubroutineReturn => {
            program.push((opcode, OperandType::None, 0));
        }
        crate::cpu::addressing_types::AddressingType::InterruptSetup => {
            program.push((opcode, OperandType::Byte, 0));
        }
        crate::cpu::addressing_types::AddressingType::ZeroPage |
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirect |
        crate::cpu::addressing_types::AddressingType::ZeroPageRMW |
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirectYIndexed => {
            program.push((opcode, OperandType::Byte, crate::cpu::Word::from(ZP_PTR)));
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexed |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedIndirect |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedRMW => {
            program.push((opcode, OperandType::Byte, crate::cpu::Word::from(ZP_PTR - test_cpu.get_register(crate::cpu::Register::X))));
        }
        _ => { panic!() }
    }
    let mut program_data: Vec<(crate::cpu::Word, crate::cpu::Byte)> = Vec::new();
    let clock_cycles: u8 = program.iter().fold(0u8, |acc, (opcode,_,_)| acc + test_cpu.get_clocks_for_addressing_type(test_cpu.get_addressing_type_for_opcode(*opcode)) );
    match addressing_type {
        crate::cpu::addressing_types::AddressingType::Accumulator | 
        crate::cpu::addressing_types::AddressingType::Immediate |
        crate::cpu::addressing_types::AddressingType::Implied | 
        crate::cpu::addressing_types::AddressingType::InterruptSetup | 
        crate::cpu::addressing_types::AddressingType::InterruptReturn | 
        crate::cpu::addressing_types::AddressingType::PcRelative | 
        crate::cpu::addressing_types::AddressingType::StackPull | 
        crate::cpu::addressing_types::AddressingType::StackPush |
        crate::cpu::addressing_types::AddressingType::SubroutineReturn => {
        }
        crate::cpu::addressing_types::AddressingType::Absolute |
        crate::cpu::addressing_types::AddressingType::AbsoluteRMW |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexed |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedRMW |
        crate::cpu::addressing_types::AddressingType::AbsoluteYIndexed |
        crate::cpu::addressing_types::AddressingType::SubroutineJump => {
            program_data.push((ABS_PTR, data));
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteIndirect |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedIndirect => {
            program_data.push((ABS_PTR, (IND_PTR << 8 >> 8) as crate::cpu::Byte));
            program_data.push((ABS_PTR + 1, (IND_PTR >> 8) as crate::cpu::Byte));
            program_data.push((IND_PTR, data));
        }
        crate::cpu::addressing_types::AddressingType::ZeroPage |
        crate::cpu::addressing_types::AddressingType::ZeroPageRMW |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexed |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedRMW |
        crate::cpu::addressing_types::AddressingType::ZeroPageYIndexed => {
            program_data.push((crate::cpu::Word::from(ZP_PTR), data));
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirect |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedIndirect => {
            program_data.push((crate::cpu::Word::from(ZP_PTR), (IND_PTR << 8 >> 8) as crate::cpu::Byte));
            program_data.push((crate::cpu::Word::from(ZP_PTR + 1), (IND_PTR >> 8) as crate::cpu::Byte));
            program_data.push((IND_PTR, data));
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirectYIndexed => {
            let offset = crate::cpu::Word::from(test_cpu.get_register(crate::cpu::Register::Y));
            program_data.push((crate::cpu::Word::from(ZP_PTR), (IND_PTR - offset << 8 >> 8) as crate::cpu::Byte));
            program_data.push((crate::cpu::Word::from(ZP_PTR + 1), (IND_PTR - offset >> 8) as crate::cpu::Byte));
            program_data.push((IND_PTR, data));
        }
    }
    let mut test_memory = setup_memory(START_PROGRAM, program, program_data);
    // This ensures cpu and memory is built for test according to the addressing mode
    assert_setup(TestCpu::clone_from_cpu(&test_cpu), test_memory, a_reg, x_reg, y_reg, sp_reg, opcode, addressing_type, data.into());
    run_test_for_x_clock(&mut test_cpu, &mut test_memory, clock_cycles);
    return (TestCpu::clone_from_cpu(&test_cpu), test_memory);
}

#[cfg(test)]
fn assert_setup(test_cpu: TestCpu, test_memory: TestMemory, a_reg: Option<crate::cpu::Byte>, x_reg: Option<crate::cpu::Byte>, y_reg: Option<crate::cpu::Byte>, sp_reg: Option<crate::cpu::Byte>, instruction: crate::cpu::Byte, addressing_type: crate::cpu::addressing_types::AddressingType, data: crate::cpu::Byte) {
    // Test CPU
    if let Some(reg_data) = a_reg {
        test_cpu.assert_register(crate::cpu::Register::A, reg_data);
    }
    if let Some(reg_data) = x_reg {
        test_cpu.assert_register(crate::cpu::Register::X, reg_data);
    }
    if let Some(reg_data) = y_reg {
        test_cpu.assert_register(crate::cpu::Register::Y, reg_data);
    }
    if let Some(reg_data) = sp_reg {
        test_cpu.assert_register(crate::cpu::Register::SP, reg_data);
    }
    // Test Memory
    test_memory.assert_byte(START_PROGRAM, instruction);
    match addressing_type {
        crate::cpu::addressing_types::AddressingType::Absolute | 
        crate::cpu::addressing_types::AddressingType::AbsoluteRMW => {
            test_memory.assert_word(START_PROGRAM + 1, ABS_PTR);
            test_memory.assert_byte(ABS_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexed |
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedRMW => {
            assert_ne!(test_cpu.x, 0);
            let offset = crate::cpu::Word::from(test_cpu.x);
            test_memory.assert_word(START_PROGRAM + 1, ABS_PTR - offset);
            test_memory.assert_byte(ABS_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteYIndexed => {
            assert_ne!(test_cpu.y, 0);
            let offset = crate::cpu::Word::from(test_cpu.y);
            test_memory.assert_word(START_PROGRAM + 1, ABS_PTR - offset);
            test_memory.assert_byte(ABS_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteIndirect => {
            test_memory.assert_word(START_PROGRAM + 1, ABS_PTR);
            test_memory.assert_word(ABS_PTR, IND_PTR);
            test_memory.assert_byte(IND_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::AbsoluteXIndexedIndirect => {
            assert_ne!(test_cpu.x, 0);
            let offset = crate::cpu::Word::from(test_cpu.x);
            test_memory.assert_word(START_PROGRAM + 1, ABS_PTR - offset);
            test_memory.assert_word(ABS_PTR, IND_PTR);
            test_memory.assert_byte(IND_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::Accumulator |
        crate::cpu::addressing_types::AddressingType::Implied |
        crate::cpu::addressing_types::AddressingType::InterruptSetup |
        crate::cpu::addressing_types::AddressingType::InterruptReturn |
        crate::cpu::addressing_types::AddressingType::StackPull |
        crate::cpu::addressing_types::AddressingType::StackPush  => {
        }
        crate::cpu::addressing_types::AddressingType::Immediate |
        crate::cpu::addressing_types::AddressingType::PcRelative => {
            test_memory.assert_byte(START_PROGRAM + 1, data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPage |
        crate::cpu::addressing_types::AddressingType::ZeroPageRMW => {
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR);
            test_memory.assert_byte(crate::cpu::Word::from(ZP_PTR), data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexed |
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedRMW => {
            assert_ne!(test_cpu.x, 0);
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR - test_cpu.x);
            test_memory.assert_byte(crate::cpu::Word::from(ZP_PTR), data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageYIndexed => {
            assert_ne!(test_cpu.y, 0);
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR - test_cpu.y);
            test_memory.assert_byte(crate::cpu::Word::from(ZP_PTR), data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirect => {
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR);
            test_memory.assert_word(crate::cpu::Word::from(ZP_PTR), IND_PTR);
            test_memory.assert_byte(IND_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageIndirectYIndexed => {
            assert_ne!(test_cpu.y, 0);
            let offset = crate::cpu::Word::from(test_cpu.y);
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR);
            test_memory.assert_word(crate::cpu::Word::from(ZP_PTR), IND_PTR - offset);
            test_memory.assert_byte(IND_PTR, data);
        }
        crate::cpu::addressing_types::AddressingType::ZeroPageXIndexedIndirect => {
            assert_ne!(test_cpu.x, 0);
            test_memory.assert_byte(START_PROGRAM + 1, ZP_PTR - test_cpu.x);
            test_memory.assert_word(crate::cpu::Word::from(ZP_PTR), IND_PTR);
            test_memory.assert_byte(IND_PTR, data);
        }
        _ => {
            panic!("Missing assertion of given addressing type")
        }
    }
}

#[cfg(test)]
fn run_test_for_x_clock(cpu: &mut crate::cpu::Cpu, test_memory: &mut TestMemory, cycles: u8) {
    let mut memory = test_memory.get_memory();
    cpu.reset(&memory);
    for _ in 0..cycles {
        cpu.exec_cycle(&mut memory)
    }
    test_memory.memory = memory;
}