#[macro_use]
extern crate bitflags;

mod cpu;
mod mem;
mod ops;

// module exports
pub use cpu::{Cpu, ProcStatus};
pub use mem::Mem;
pub use ops::*;

// type aliases
pub(self) type Byte = u8;
pub(self) type Word = u16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_is_initialized_correctly() {
        let cpu = Cpu::new();

        assert_eq!(cpu.pc, 0xFFFC);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.ps, ProcStatus::empty());
    }

    #[test]
    fn cpu_and_memory_is_reset_correctly() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;
        cpu.sp = 0x01;
        cpu.a = 0x02;
        cpu.x = 0x03;
        cpu.y = 0x04;
        cpu.ps = ProcStatus::all();

        let mut cpu = Cpu::new();
        let mut mem = Mem::new();
        mem[0x0000] = 0xDE;
        mem[0x0001] = 0xAD;
        mem[0x0002] = 0xBE;
        mem[0x0003] = 0xEF;

        cpu.reset(&mut mem);

        assert_eq!(cpu.pc, 0xFFFC);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.ps, ProcStatus::empty());
        assert_eq!(mem.data, [OP_NOP; 1024 * 64]);
    }

    #[test]
    fn test_op_lda_im() {
        let mut cpu = Cpu::new();
        let mut mem = Mem::new();
        let mut cycles = 2;

        mem[0xFFFC] = OP_LDA_IM;
        mem[0xFFFD] = 0x01;

        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
        assert_eq!(cpu.sp, 0x00);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.ps, ProcStatus::empty());
        assert_eq!(cycles, 0);
    }

    #[test]
    fn test_op_lda_im_negative() {
        let mut cpu = Cpu::new();
        let mut mem = Mem::new();
        let mut cycles = 2;

        mem[0xFFFC] = OP_LDA_IM;
        mem[0xFFFD] = 0xF1;

        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
        assert_eq!(cpu.sp, 0x00);
        assert_eq!(cpu.a, 0xF1);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.ps, ProcStatus::N);
        assert_eq!(cycles, 0);
    }

    #[test]
    fn test_op_lda_im_zero() {
        let mut cpu = Cpu::new();
        let mut mem = Mem::new();
        let mut cycles = 2;

        mem[0xFFFC] = OP_LDA_IM;
        mem[0xFFFD] = 0x00;

        cpu.execute(&mut cycles, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
        assert_eq!(cpu.sp, 0x00);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.ps, ProcStatus::Z);
        assert_eq!(cycles, 0);
    }
}
