use byteorder::{LittleEndian, ByteOrder};

// NOTE: ALL 16-bit data is written in the Little Endian form in memory
// Example: Storing hexadecimal word ABCD in the address 0x8000
// would have the byte CD in address 0x8000 and the byte AB in address 0x8001

use crate::cpu::{Byte, Word};

#[derive(Clone, Copy)]
pub struct Memory {
    data: [Byte; usize::pow(2,16)]
}

impl Memory {
    pub fn build_memory() -> Memory {
        return Memory {
            data: [0x00; usize::pow(2, 16)],
        }
    }

    pub fn read_byte(&self, address: Word) -> Byte {
        let index = usize::from(address);
        let result = Byte::from(self.data[index]);
        return result;
    }

    pub fn read_word(&self, address: Word) -> Word {
        let data: [Byte; 2] = [self.data[usize::from(address)], self.data[usize::from(address) + 1]];
        return Word::from(LittleEndian::read_u16(&data));
    }

    pub fn write_byte(&mut self, address: Word, data: Byte) {
        let index = usize::from(address);
        self.data[index] = data;
    }

    pub fn write_word(&mut self, address: Word, data: Word) {
        let mut data_arr = [0,0];
        LittleEndian::write_u16(&mut data_arr, data);
        self.write_byte(address, data_arr[0]);
        self.write_byte(address+1, data_arr[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_ADDRESS: usize = 0x8000;
    const DATA_BYTE: Byte = 0xEA;
    const DATA_WORD: Word = 0xEAAE;

    #[test]
    fn test_read_byte() {
        // setup of memory
        let mut mem_array: [Byte; usize::pow(2,16)] = [0; usize::pow(2,16)];
        mem_array[TEST_ADDRESS] = DATA_BYTE;
        let memory = Memory { data: mem_array };
        assert_eq!(memory.read_byte(TEST_ADDRESS as Word), DATA_BYTE);
    }

    #[test]
    fn test_read_word() {
        // setup of memory
        let mut mem_array: [Byte; usize::pow(2,16)] = [0; usize::pow(2,16)];
        let mut data_buff: [Byte; 2] = [0,0];
        LittleEndian::write_u16(&mut data_buff, DATA_WORD);
        mem_array[TEST_ADDRESS] = data_buff[0];
        mem_array[TEST_ADDRESS + 1] = data_buff[1];
        let memory = Memory { data: mem_array };
        assert_eq!(memory.read_word(TEST_ADDRESS as Word), DATA_WORD);
    }

    #[test]
    fn test_write_byte() {
        let mem_array: [Byte; usize::pow(2,16)] = [0; usize::pow(2,16)];
        let mut memory = Memory { data: mem_array };
        memory.write_byte(TEST_ADDRESS as Word, DATA_BYTE);
        assert_eq!(memory.data[TEST_ADDRESS], DATA_BYTE);
    }

    #[test]
    fn test_write_word() {
        let mem_array: [Byte; usize::pow(2,16)] = [0; usize::pow(2,16)];
        let mut memory = Memory { data: mem_array };
        memory.write_word(TEST_ADDRESS as Word, DATA_WORD);
        let mut data_buff: [Byte; 2] = [0,0];
        LittleEndian::write_u16(&mut data_buff, DATA_WORD);
        assert_eq!(memory.data[TEST_ADDRESS], data_buff[0]);
        assert_eq!(memory.data[TEST_ADDRESS + 1], data_buff[1]);
    }
}