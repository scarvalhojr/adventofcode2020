use super::*;

use std::str::FromStr;

struct Bitmask {
    or_mask: u64,
    and_mask: u64,
}

impl Default for Bitmask {
    fn default() -> Self {
        Self {
            or_mask: 0,
            and_mask: !0,
        }
    }
}

impl Bitmask {
    fn apply(&self, value: u64) -> u64 {
        (value | self.or_mask) & self.and_mask
    }
}

#[derive(Default)]
struct Program {
    memory: HashMap<u64, u64>,
    bitmask: Bitmask,
}

impl Program {
    fn run(&mut self, instructions: &[Instruction]) -> Result<(), String> {
        for instr in instructions {
            match instr {
                Instruction::Mask(mask) => self.bitmask = mask.parse()?,
                &Instruction::Mem(address, value) => {
                    self.memory.insert(address, self.bitmask.apply(value));
                }
            }
        }
        Ok(())
    }

    fn mem_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn part1(instructions: &[Instruction]) -> Option<u64> {
    let mut program = Program::default();
    program.run(instructions).ok();
    Some(program.mem_sum())
}

impl FromStr for Bitmask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let or_mask =
            s.chars()
                .rev()
                .zip(0..)
                .fold(0, |mask, (ch, bit)| match ch {
                    '1' => mask | 2_u64.pow(bit),
                    _ => mask,
                });
        let and_mask =
            !s.chars()
                .rev()
                .zip(0..)
                .fold(0, |mask, (ch, bit)| match ch {
                    '0' => mask | 2_u64.pow(bit),
                    _ => mask,
                });

        Ok(Self { or_mask, and_mask })
    }
}
