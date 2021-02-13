use super::*;
use std::cmp::min;

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

#[derive(Debug)]
pub struct Cpu {
    pub pc: Word, // program counter
    pub sp: Byte, // stack pointer

    // registers
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,

    pub ps: ProcStatus, // processor status
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0xFFFC,
            sp: 0x00,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            ps: ProcStatus::empty(),
        }
    }

    pub fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.ps = ProcStatus::empty();
        mem.initialize();
    }

    pub fn execute(&mut self, mut cycles: &mut i32, mem: &mut Mem) {
        while *cycles > 0 {
            let ins: Byte = self.fetch_byte(&mut cycles, mem);

            match ins {
                OP_JSR => {
                    let sub_addr = self.fetch_word(&mut cycles, mem);
                    mem.write_word(
                        &mut cycles,
                        Cpu::get_stack_addr(self.sp),
                        self.pc.wrapping_sub(1),
                    );
                    self.sp += self.sp.wrapping_add(1);
                    self.pc = sub_addr;
                    Cpu::decrement_cycles(&mut cycles, 1);
                }
                OP_LDA_IM => {
                    self.a = self.fetch_byte(&mut cycles, mem);
                    self.lda_set_status();
                }
                OP_LDA_ZP => {
                    let zero_page_addr = self.fetch_byte(&mut cycles, mem);
                    self.a = Cpu::read_byte(&mut cycles, mem, zero_page_addr);
                    self.lda_set_status();
                }
                OP_LDA_ZPX => {
                    let zero_page_addr = self.fetch_byte(&mut cycles, mem).wrapping_add(self.x);
                    *cycles -= 1;
                    self.a = Cpu::read_byte(&mut cycles, mem, zero_page_addr);
                    self.lda_set_status();
                }
                OP_NOP => {
                    Cpu::decrement_cycles(&mut cycles, 1);
                }
                _ => panic!("UNKNOWN OP!"),
            }
        }
    }

    fn lda_set_status(&mut self) {
        self.ps.set(ProcStatus::Z, self.a == 0); // is zero
        self.ps.set(ProcStatus::N, self.a & 0b10000000 > 0) // is negative
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn decrement_cycles(cycles: &mut i32, cost: i32) {
        *cycles -= min(*cycles, cost);
    }

    fn read_byte(cycles: &mut i32, mem: &Mem, address: Byte) -> Byte {
        let data: Byte = mem[address as usize];
        Cpu::decrement_cycles(cycles, 1);
        data
    }

    fn fetch_byte(&mut self, cycles: &mut i32, mem: &Mem) -> Byte {
        let data: Byte = mem[self.pc as usize];
        self.increment_pc();
        Cpu::decrement_cycles(cycles, 1);
        data
    }

    fn get_stack_addr(sp: Byte) -> Word {
        let stack_addr: Word = 0x0100;
        stack_addr | sp as Word
    }

    fn fetch_word(&mut self, cycles: &mut i32, mem: &Mem) -> Word {
        // 6502 is little endian
        let mut data: Word = mem[self.pc as usize] as Word;
        self.increment_pc();

        data |= (mem[self.pc as usize] as Word) << 8;
        self.increment_pc();

        Cpu::decrement_cycles(cycles, 2);
        data
    }
}
