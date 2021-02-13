use super::*;
use std::ops::Index;
use std::ops::IndexMut;

const MAX_MEM: usize = 1024 * 64;

#[derive(Debug)]
pub struct Mem {
    pub data: [Byte; MAX_MEM],
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            data: [OP_NOP; MAX_MEM],
        }
    }

    pub fn initialize(&mut self) {
        self.data = [OP_NOP; MAX_MEM];
    }

    pub fn write_word(&mut self, cycles: &mut i32, addr: Word, data: Word) {
        self[addr as usize] = data as Byte;
        self[(addr + 1) as usize] = (data >> 8) as Byte;
        Cpu::decrement_cycles(cycles, 2);
    }
}

impl Index<usize> for Mem {
    type Output = Byte;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
