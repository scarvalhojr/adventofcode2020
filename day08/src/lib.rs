#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

type Program = [Instruction];

#[derive(Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum State {
    Error,
    Loop(i32),
    Completed(i32),
}

struct Execution<'a> {
    program: &'a Program,
    prog_counter: i32,
    accummulator: i32,
}

impl<'a> Execution<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            program,
            prog_counter: 0,
            accummulator: 0,
        }
    }

    fn run(&mut self) -> State {
        if self.prog_counter != 0 {
            return State::Error;
        }

        let mut visited = HashSet::new();
        visited.insert(0);

        while let Some(pg) = self.step() {
            if !visited.insert(pg) {
                return State::Loop(self.accummulator);
            }
            if pg == self.program.len() {
                return State::Completed(self.accummulator);
            }
        }
        State::Error
    }

    fn get_prog_counter(&self) -> Option<usize> {
        usize::try_from(self.prog_counter).ok()
    }

    fn step(&mut self) -> Option<usize> {
        let pg = self.get_prog_counter()?;
        match self.program.get(pg)? {
            Instruction::Acc(arg) => {
                self.accummulator += arg;
                self.prog_counter += 1;
            }
            Instruction::Jmp(arg) => {
                self.prog_counter += arg;
            }
            Instruction::Nop(_) => {
                self.prog_counter += 1;
            }
        };
        self.get_prog_counter()
    }
}

pub fn part1(program: &Program) -> Option<i32> {
    match Execution::new(program).run() {
        State::Loop(acc) => Some(acc),
        _ => None,
    }
}

pub fn part2(program: &Program) -> Option<i32> {
    let mut mod_prog = program.to_vec();
    for (pos, instruction) in program.iter().enumerate() {
        match instruction {
            Instruction::Jmp(arg) => mod_prog[pos] = Instruction::Nop(*arg),
            Instruction::Nop(arg) => mod_prog[pos] = Instruction::Jmp(*arg),
            _ => continue,
        };

        // Try the modified program
        if let State::Completed(acc) = Execution::new(&mod_prog).run() {
            return Some(acc);
        }

        // Restore the original instruction
        mod_prog[pos] = instruction.clone();
    }
    // No modified program ever completed
    None
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(?P<op>\w+) (?P<arg>[\+\-]\d+)$").unwrap();
        }
        let captures =
            REGEX.captures(s).ok_or_else(|| "Invalid instruction")?;
        let arg = captures
            .name("arg")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|err| format!("Invalid argument: {}", err))?;
        match captures.name("op").unwrap().as_str() {
            "acc" => Ok(Self::Acc(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            "nop" => Ok(Self::Nop(arg)),
            op => Err(format!("Unknown operation: {}", op)),
        }
    }
}
