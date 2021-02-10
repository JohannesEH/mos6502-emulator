#[macro_use]
extern crate bitflags;

use std::ops::Index;
use std::ops::IndexMut;

// http://www.obelisk.me.uk/6502/

// type aliases
type Byte = u8;
type Word = u16;

// opcodes
const INS_LDA_IM: Byte = 0xA9;

bitflags! {
    pub struct ProcStatus: u8 {
        const C = 0b00000001; // carry flag
        const Z = 0b00000010; // zero flag
        const I = 0b00000100; // interrupt disable
        const D = 0b00001000; // decimal mode
        const B = 0b00010000; // break command
        const V = 0b00100000; // overflow flag
        const N = 0b01000000; // negative flag
    }
}

const MAX_MEM: usize = 1024 * 64;

#[derive(Debug)]
pub struct Mem {
    data: [Byte; MAX_MEM],
}

impl Mem {
    pub fn new() -> Mem {
        Mem { data: [0; MAX_MEM] }
    }

    pub fn initialize(&mut self) {
        self.data = [0; MAX_MEM];
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

#[derive(Debug)]
pub struct Cpu {
    pc: Word, // program counter
    sp: Word, // stack pointer

    // registers
    a: Byte,
    x: Byte,
    y: Byte,

    ps: ProcStatus, // processor status
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0xFFFC,
            sp: 0x0100,
            a: 0,
            x: 0,
            y: 0,
            ps: ProcStatus::empty(),
        }
    }

    pub fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.ps = ProcStatus::empty();
        mem.initialize();
    }

    pub fn execute(&mut self, mut cycles: u32, mem: &mut Mem) {
        while cycles > 0 {
            let ins: Byte = self.fetch_byte(&mut cycles, mem);

            match ins {
                INS_LDA_IM => {
                    let arg = self.fetch_byte(&mut cycles, mem);
                    self.a = arg;
                    self.ps.set(ProcStatus::Z, self.a == 0); // is zero
                    self.ps.set(ProcStatus::N, self.a & 0b10000000 > 0) // is negative
                }
                _ => panic!("UNKNOWN OP!"),
            }
        }
    }

    fn fetch_byte(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let data: Byte = mem[self.pc as usize];

        println!(
            "Fetched instruction 0x{:02X?} from address 0x{:04X?}",
            data, self.pc
        );

        self.pc += 1;
        *cycles -= 1;
        data
    }
}

fn main() {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();

    cpu.reset(&mut mem);
    mem[0xFFFC] = INS_LDA_IM;
    mem[0xFFFD] = 0x01;
    cpu.execute(2, &mut mem);

    // println!("{:02X?}", mem);
    println!("{:04X?}", cpu);
}
