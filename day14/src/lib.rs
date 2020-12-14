#[macro_use]
extern crate lazy_static;

pub mod part1;
pub mod part2;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(concat!(
                r"^((mask = (?P<mask>[01X]{36}))|",
                r"(mem\[(?P<addr>\d+)\] = (?P<val>\d+)))$",
            ))
            .unwrap();
        }
        let captures =
            REGEX.captures(s).ok_or_else(|| "Invalid instruction")?;
        if let Some(mask) = captures.name("mask") {
            Ok(Self::Mask(mask.as_str().to_string()))
        } else {
            let address = captures
                .name("addr")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| format!("Invalid address: {}", err))?;
            let value = captures
                .name("val")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| format!("Invalid value: {}", err))?;
            Ok(Self::Mem(address, value))
        }
    }
}
