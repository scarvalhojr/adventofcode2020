#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn valid_v1(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&required| self.fields.contains_key(required))
    }

    fn valid_v2(&self) -> bool {
        self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
    }

    fn valid_range(val_str: &str, min_val: u32, max_val: u32) -> Option<bool> {
        val_str
            .parse::<u32>()
            .map(|value| value >= min_val && value <= max_val)
            .ok()
    }

    fn valid_year(&self, key: &str, min_val: u32, max_val: u32) -> bool {
        self.fields
            .get(key)
            .and_then(|value| Self::valid_range(value, min_val, max_val))
            .unwrap_or(false)
    }

    fn valid_byr(&self) -> bool {
        self.valid_year("byr", 1920, 2002)
    }

    fn valid_iyr(&self) -> bool {
        self.valid_year("iyr", 2010, 2020)
    }

    fn valid_eyr(&self) -> bool {
        self.valid_year("eyr", 2020, 2030)
    }

    fn valid_hgt(&self) -> bool {
        self.fields
            .get("hgt")
            .and_then(|height| {
                if let Some(val_cm) = height.strip_suffix("cm") {
                    Self::valid_range(val_cm, 150, 193)
                } else if let Some(val_in) = height.strip_suffix("in") {
                    Self::valid_range(val_in, 59, 76)
                } else {
                    None
                }
            })
            .unwrap_or(false)
    }

    fn valid_hcl(&self) -> bool {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        self.fields
            .get("hcl")
            .map(|value| REGEX.is_match(&value))
            .unwrap_or(false)
    }

    fn valid_ecl(&self) -> bool {
        let valid_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        self.fields
            .get("ecl")
            .map(|colour| valid_colours.iter().any(|valid| valid == colour))
            .unwrap_or(false)
    }

    fn valid_pid(&self) -> bool {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        self.fields
            .get("pid")
            .map(|value| REGEX.is_match(&value))
            .unwrap_or(false)
    }
}

pub fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.valid_v1())
        .count()
}

pub fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.valid_v2())
        .count()
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split_whitespace()
            .map(|field| {
                let mut parts = field.split(':').map(str::to_string).take(2);
                match (parts.next(), parts.next(), parts.next()) {
                    (Some(key), Some(value), None) => {
                        // TODO: check if key is valid?
                        Ok((key, value))
                    }
                    _ => Err(format!("Invalid field: {}", field)),
                }
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self { fields })
    }
}
