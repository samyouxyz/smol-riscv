pub mod cpu;
pub mod inst;

use std::io;

use crate::cpu::Cpu;

fn main() -> io::Result<()> {
    Cpu::run();
    Ok(())
}
