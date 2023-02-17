use byteorder::{LittleEndian, ByteOrder};

use crate::{cpu::Cpu, memory::Memory};

use super::{Byte, Word};

pub (in crate::cpu) type Instruction = fn(&mut Cpu, &mut Memory);

use std::{collections::HashMap};

use crate::{cpu::{addressing_types::AddressingType, instructions}};

// Any instructions implemented here will have to coordinate with TCU and IR to know what to to on a given clock.
// Many instructions will follow the following implementation:
//   1. Test instruction register to check addressing type
//   2. Test timing control unit to check which clock we are on

impl crate::cpu::Cpu {
    pub(in crate::cpu) fn build_instruction_set() -> HashMap<Byte, Instruction> {
        let mut instruction_set: HashMap<Byte, Instruction> = HashMap::new();
        instruction_set.extend(instructions::adc::build_instruction_set());
        instruction_set.extend(instructions::and::build_instruction_set());
        instruction_set.extend(instructions::asl::build_instruction_set());
        instruction_set.extend(instructions::bcc::build_instruction_set());
        instruction_set.extend(instructions::bcs::build_instruction_set());
        instruction_set.extend(instructions::beq::build_instruction_set());
        instruction_set.extend(instructions::bit::build_instruction_set());
        instruction_set.extend(instructions::bmi::build_instruction_set());
        instruction_set.extend(instructions::bne::build_instruction_set());
        instruction_set.extend(instructions::bpl::build_instruction_set());
        instruction_set.extend(instructions::bra::build_instruction_set());
        instruction_set.extend(instructions::brk::build_instruction_set());
        instruction_set.extend(instructions::bvc::build_instruction_set());
        instruction_set.extend(instructions::bvs::build_instruction_set());
        instruction_set.extend(instructions::clc::build_instruction_set());
        instruction_set.extend(instructions::cld::build_instruction_set());
        instruction_set.extend(instructions::cli::build_instruction_set());
        instruction_set.extend(instructions::clv::build_instruction_set());
        instruction_set.extend(instructions::cmp::build_instruction_set());
        instruction_set.extend(instructions::cpx::build_instruction_set());
        instruction_set.extend(instructions::cpy::build_instruction_set());
        instruction_set.extend(instructions::dec::build_instruction_set());
        instruction_set.extend(instructions::dex::build_instruction_set());
        instruction_set.extend(instructions::dey::build_instruction_set());
        instruction_set.extend(instructions::eor::build_instruction_set());
        instruction_set.extend(instructions::inc::build_instruction_set());
        instruction_set.extend(instructions::inx::build_instruction_set());
        instruction_set.extend(instructions::iny::build_instruction_set());
        instruction_set.extend(instructions::jmp::build_instruction_set());
        instruction_set.extend(instructions::jsr::build_instruction_set());
        instruction_set.extend(instructions::lda::build_instruction_set());
        instruction_set.extend(instructions::ldx::build_instruction_set());
        instruction_set.extend(instructions::ldy::build_instruction_set());
        instruction_set.extend(instructions::lsr::build_instruction_set());
        instruction_set.extend(instructions::nop::build_instruction_set());
        instruction_set.extend(instructions::ora::build_instruction_set());
        instruction_set.extend(instructions::pha::build_instruction_set());
        instruction_set.extend(instructions::php::build_instruction_set());
        instruction_set.extend(instructions::phx::build_instruction_set());
        instruction_set.extend(instructions::phy::build_instruction_set());
        instruction_set.extend(instructions::pla::build_instruction_set());
        instruction_set.extend(instructions::plp::build_instruction_set());
        instruction_set.extend(instructions::plx::build_instruction_set());
        instruction_set.extend(instructions::ply::build_instruction_set());
        instruction_set.extend(instructions::rol::build_instruction_set());
        instruction_set.extend(instructions::ror::build_instruction_set());
        instruction_set.extend(instructions::rti::build_instruction_set());
        instruction_set.extend(instructions::rts::build_instruction_set());
        instruction_set.extend(instructions::sbc::build_instruction_set());
        instruction_set.extend(instructions::sec::build_instruction_set());
        instruction_set.extend(instructions::sed::build_instruction_set());
        instruction_set.extend(instructions::sei::build_instruction_set());
        instruction_set.extend(instructions::sta::build_instruction_set());
        instruction_set.extend(instructions::stx::build_instruction_set());
        instruction_set.extend(instructions::sty::build_instruction_set());
        instruction_set.extend(instructions::stz::build_instruction_set());
        instruction_set.extend(instructions::tax::build_instruction_set());
        instruction_set.extend(instructions::tay::build_instruction_set());
        instruction_set.extend(instructions::trb::build_instruction_set());
        instruction_set.extend(instructions::tsb::build_instruction_set());
        instruction_set.extend(instructions::tsx::build_instruction_set());
        instruction_set.extend(instructions::txa::build_instruction_set());
        instruction_set.extend(instructions::txs::build_instruction_set());
        instruction_set.extend(instructions::tya::build_instruction_set());
        return instruction_set;
    }       

    pub(in crate::cpu) fn build_addressing_type() -> HashMap<Byte, AddressingType>  {
        let mut addressing_map = HashMap::new();
        addressing_map.extend(instructions::adc::build_addressing_type());
        addressing_map.extend(instructions::and::build_addressing_type());
        addressing_map.extend(instructions::asl::build_addressing_type());
        addressing_map.extend(instructions::bcc::build_addressing_type());
        addressing_map.extend(instructions::bcs::build_addressing_type());
        addressing_map.extend(instructions::beq::build_addressing_type());
        addressing_map.extend(instructions::bit::build_addressing_type());
        addressing_map.extend(instructions::bmi::build_addressing_type());
        addressing_map.extend(instructions::bne::build_addressing_type());
        addressing_map.extend(instructions::bpl::build_addressing_type());
        addressing_map.extend(instructions::bra::build_addressing_type());
        addressing_map.extend(instructions::brk::build_addressing_type());
        addressing_map.extend(instructions::bvc::build_addressing_type());
        addressing_map.extend(instructions::bvs::build_addressing_type());
        addressing_map.extend(instructions::clc::build_addressing_type());
        addressing_map.extend(instructions::cld::build_addressing_type());
        addressing_map.extend(instructions::cli::build_addressing_type());
        addressing_map.extend(instructions::clv::build_addressing_type());
        addressing_map.extend(instructions::cmp::build_addressing_type());
        addressing_map.extend(instructions::cpx::build_addressing_type());
        addressing_map.extend(instructions::cpy::build_addressing_type());
        addressing_map.extend(instructions::dec::build_addressing_type());
        addressing_map.extend(instructions::dex::build_addressing_type());
        addressing_map.extend(instructions::dey::build_addressing_type());
        addressing_map.extend(instructions::eor::build_addressing_type());
        addressing_map.extend(instructions::inc::build_addressing_type());
        addressing_map.extend(instructions::inx::build_addressing_type());
        addressing_map.extend(instructions::iny::build_addressing_type());
        addressing_map.extend(instructions::jmp::build_addressing_type());
        addressing_map.extend(instructions::jsr::build_addressing_type());
        addressing_map.extend(instructions::lda::build_addressing_type());
        addressing_map.extend(instructions::ldx::build_addressing_type());
        addressing_map.extend(instructions::ldy::build_addressing_type());
        addressing_map.extend(instructions::lsr::build_addressing_type());
        addressing_map.extend(instructions::nop::build_addressing_type());
        addressing_map.extend(instructions::ora::build_addressing_type());
        addressing_map.extend(instructions::pha::build_addressing_type());
        addressing_map.extend(instructions::php::build_addressing_type());
        addressing_map.extend(instructions::phx::build_addressing_type());
        addressing_map.extend(instructions::phy::build_addressing_type());
        addressing_map.extend(instructions::pla::build_addressing_type());
        addressing_map.extend(instructions::plp::build_addressing_type());
        addressing_map.extend(instructions::plx::build_addressing_type());
        addressing_map.extend(instructions::ply::build_addressing_type());
        addressing_map.extend(instructions::rol::build_addressing_type());
        addressing_map.extend(instructions::ror::build_addressing_type());
        addressing_map.extend(instructions::rti::build_addressing_type());
        addressing_map.extend(instructions::rts::build_addressing_type());
        addressing_map.extend(instructions::sbc::build_addressing_type());
        addressing_map.extend(instructions::sec::build_addressing_type());
        addressing_map.extend(instructions::sed::build_addressing_type());
        addressing_map.extend(instructions::sei::build_addressing_type());
        addressing_map.extend(instructions::sta::build_addressing_type());
        addressing_map.extend(instructions::stx::build_addressing_type());
        addressing_map.extend(instructions::sty::build_addressing_type());
        addressing_map.extend(instructions::stz::build_addressing_type());
        addressing_map.extend(instructions::tax::build_addressing_type());
        addressing_map.extend(instructions::tay::build_addressing_type());
        addressing_map.extend(instructions::trb::build_addressing_type());
        addressing_map.extend(instructions::tsb::build_addressing_type());
        addressing_map.extend(instructions::tsx::build_addressing_type());
        addressing_map.extend(instructions::txa::build_addressing_type());
        addressing_map.extend(instructions::txs::build_addressing_type());
        addressing_map.extend(instructions::tya::build_addressing_type());
        return addressing_map;
    }
}


// process status tests
pub(in crate::cpu)fn test_carry(data: Word) -> bool {
    let mut data_split: [Byte; 2] = [0,0];
    LittleEndian::write_u16(&mut data_split, data);
    return data_split[1] > 0
}

// Further details for implementation on http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
pub(in crate::cpu)fn test_overflow(orig_a_reg: Byte, orig_data: Byte, result: Byte) -> bool {
    return (orig_a_reg ^ result) & (orig_data ^ result) & 0x80 > 0;
}

// Operations
pub(in crate::cpu) fn add(a_reg: Byte, data: Byte, carry_status: bool) -> Word {
    return a_reg as Word + data as Word + if carry_status { 1 } else { 0 };
}

pub(in crate::cpu) fn offset_word_signed_byte(addr: Word, offset: Byte) -> Word {
    let mut buffer: [Byte; 2] = [0,0];
    LittleEndian::write_u16(&mut buffer, addr);
    let top_bits = buffer[1];
    let bottom_bits = buffer[0];
    LittleEndian::write_u16(&mut buffer, Word::from(bottom_bits) + Word::from(offset));
    buffer[1] = top_bits;
    return LittleEndian::read_u16(&buffer);
}

pub(in crate::cpu) fn offset_byte_signed_byte(byte: Byte, offset: Byte) -> Byte {
    let new_word: Word = offset_word_signed_byte(0x0000 + Word::from(byte), offset);
    let mut new_byte_buff: [Byte; 2] = [0,0];
    LittleEndian::write_u16(&mut new_byte_buff, new_word);
    return new_byte_buff[0];
}