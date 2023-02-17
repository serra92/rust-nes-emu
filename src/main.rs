mod cpu;
mod memory;
pub mod test_utils;

use cpu::Cpu;

use crate::memory::Memory;

fn main() {
    let mut memory: Memory = Memory::build_memory();
    // init inline memory
    for address in 0..0xFFFF {
        memory.write_byte(address, 0xEA)
    }

    // Zero Page
    memory.write_byte(0x0010, 0b1000_0000);
    memory.write_byte(0x0011, 0b0100_0000);
    // Data
    memory.write_byte(0x9002, 0xF0);
    memory.write_byte(0x9003, 0x00);
    // Reset vector
    memory.write_word(0xFFFC, 0x8000);
    // Program
    memory.write_byte(0x8000, 0x88);

    let mut cpu: Cpu = Cpu::build_cpu();

    cpu.reset(&memory);

    for _i in 0..10 {
        cpu.exec_cycle(&mut memory);
    }
}

// -------------- AUDIO EXAMPLE ----------------

// use rodio::{OutputStream, Sink};

// fn main() {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();

//     sink.sleep_until_end();
// }