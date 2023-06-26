use crate::inst::Inst;

use std::{env, fs::File, io::Read};

const MEM_SIZE: u32 = 1024 * 1024 * 64; // 64MB

pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub mem: Vec<u8>,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs: [u32; 32] = [0; 32];
        regs[1] = MEM_SIZE;
        Self {
            regs,
            pc: 0,
            mem: code,
        }
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;

        // little endian
        (self.mem[index] as u32)
            | (self.mem[index + 1] as u32) << 8
            | (self.mem[index] as u32) << 16
            | (self.mem[index] as u32) << 24
    }

    pub fn decode(&self, inst: u32) -> Inst {
        let opcode = inst & 0x7f;
        match opcode {
            0x13 => {
                let funct3 = (inst >> 12) & 0x7;
                match funct3 {
                    0x0 => Inst::ADDI,
                    _ => panic!("not implemented opcode {:x}", opcode),
                }
            }
            0x33 => Inst::ADD,
            _ => panic!("not implemented opcode {:x}", opcode),
        }
    }

    pub fn execute(&mut self, inst: u32) {
        let opcode: Inst = self.decode(inst);
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        // execute instructions
        match opcode {
            Inst::ADD => {
                self.regs[rd] = self.regs[rs1] + self.regs[rs2];
            }
            Inst::ADDI => {
                let imm = sign_extend((inst >> 20) & 0xfff, 12);
                self.regs[rd] = self.regs[rs1] + imm;
            }
        }
    }

    pub fn dump(&self) {
        println!("\nRegisters:");
        for i in 0..32 {
            println!("x{} = 0x{:x}", i, self.regs[i]);
        }
    }

    pub fn run() {
        let args: Vec<String> = env::args().collect();
        let mut code = Vec::new();
        let mut file = match File::open(&args[1]) {
            Ok(file) => file,
            Err(error) => panic!("Failed to open file: {}", error),
        };

        match file.read_to_end(&mut code) {
            Ok(_) => {
                println!("File contents:\n{:?}", code);
            }
            Err(error) => {
                panic!("Failed to read file: {}", error);
            }
        }

        let mut cpu = Cpu::new(code);
        while cpu.pc < cpu.mem.len() as u32 {
            let inst = cpu.fetch();
            cpu.pc += 4;
            cpu.execute(inst);
        }

        cpu.dump();
    }
}

fn sign_extend(x: u32, bit: usize) -> u32 {
    if (x >> (bit - 1)) & 1 == 1 {
        (0xffffffff << bit) | x
    } else {
        x
    }
}
