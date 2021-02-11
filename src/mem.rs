use super::*;
use std::ops::Index;
use std::ops::IndexMut;

const MAX_MEM: usize = 1024 * 64;

#[derive(Debug)]
pub struct Mem {
    data: [Byte; MAX_MEM],
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
