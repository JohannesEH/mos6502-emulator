#[macro_use]
extern crate bitflags;

mod cpu;
mod mem;

// module exports
pub use cpu::Cpu;
pub use mem::Mem;

// type aliases
pub(self) type Byte = u8;
pub(self) type Word = u16;

// opcodes
pub const OP_LDA_IM: Byte = 0xA9;
pub const OP_NOP: Byte = 0xEA;
