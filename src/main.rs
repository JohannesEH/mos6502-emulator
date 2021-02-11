extern crate mos6502;
use mos6502::{Cpu, Mem};

// http://www.obelisk.me.uk/6502/

fn main() {
    let mut mem = Mem::new();
    let mut cpu = Cpu::new();

    cpu.reset(&mut mem);
    // mem[cpu.pc as usize] = OP_LDA_IM;
    // mem[(cpu.pc + 1) as usize] = 0x00;
    cpu.execute(20, &mut mem);

    // println!("{:02X?}", mem);
    println!("{:04X?}", cpu);
}
