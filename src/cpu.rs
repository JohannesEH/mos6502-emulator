use super::*;

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
    pub sp: Word, // stack pointer

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
                OP_LDA_IM => {
                    let arg = self.fetch_byte(&mut cycles, mem);
                    self.a = arg;
                    self.ps.set(ProcStatus::Z, self.a == 0); // is zero
                    self.ps.set(ProcStatus::N, self.a & 0b10000000 > 0) // is negative
                }
                OP_NOP => {
                    cycles -= 2;
                }
                _ => panic!("UNKNOWN OP!"),
            }
        }
    }

    fn fetch_byte(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let data: Byte = mem[self.pc as usize];

        // println!(
        //     "Fetched instruction 0x{:02X?} from address 0x{:04X?}",
        //     data, self.pc
        // );

        self.pc += 1;
        *cycles -= 1;
        data
    }
}
