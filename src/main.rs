extern crate mos6502;
use mos6502::{Cpu, Mem, *};

// http://www.obelisk.me.uk/6502/

fn main() {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();
    let mut cycles: i32 = 9;

    cpu.reset(&mut mem);

    cpu.x = 0x01;
    mem[0xFFFC] = OP_JSR;
    mem[0xFFFD] = 0x42;
    mem[0xFFFE] = 0x42;
    mem[0x4242] = OP_LDA_IM;
    mem[0x4243] = 0xDA;

    cpu.execute(&mut cycles, &mut mem);

    // println!("{:02X?}", mem);
    println!("{:04X?}", cpu);
}
