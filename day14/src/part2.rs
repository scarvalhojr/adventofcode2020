use super::*;

#[derive(Default)]
struct Bitmask {
    or_mask: u64,
    xor_masks: Vec<u64>,
}

impl Bitmask {
    fn apply(&self, value: u64) -> Vec<u64> {
        let mut mod_values = vec![value | self.or_mask];
        for mask in self.xor_masks.iter() {
            for index in 0..mod_values.len() {
                mod_values.push(mod_values[index] ^ mask);
            }
        }
        mod_values
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
                    for mod_address in self.bitmask.apply(address) {
                        self.memory.insert(mod_address, value);
                    }
                }
            }
        }
        Ok(())
    }

    fn mem_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn part2(instructions: &[Instruction]) -> Option<u64> {
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
        let xor_masks = s
            .chars()
            .rev()
            .zip(0..)
            .filter(|&(ch, _)| ch == 'X')
            .map(|(_, bit)| 2_u64.pow(bit))
            .collect::<Vec<_>>();

        Ok(Self { or_mask, xor_masks })
    }
}
