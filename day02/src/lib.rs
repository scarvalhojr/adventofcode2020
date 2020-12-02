use regex::Regex;
use std::str::FromStr;

struct PasswordRule {
    first_num: usize,
    second_num: usize,
    character: char,
}

pub struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

pub fn part1(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|e| e.is_valid_v1()).count()
}

pub fn part2(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|e| e.is_valid_v2()).count()
}

impl PasswordEntry {
    fn is_valid_v1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&ch| ch == self.rule.character)
            .count();
        count >= self.rule.first_num && count <= self.rule.second_num
    }

    fn is_valid_v2(&self) -> bool {
        let match1 = self
            .password
            .chars()
            .nth(self.rule.first_num - 1)
            .map(|ch| ch == self.rule.character)
            .unwrap_or(false);
        let match2 = self
            .password
            .chars()
            .nth(self.rule.second_num - 1)
            .map(|ch| ch == self.rule.character)
            .unwrap_or(false);
        match1 ^ match2
    }
}

impl FromStr for PasswordRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"^(\d+)-(\d+)\s(?P<ch>.)$")
            .unwrap()
            .captures(s)
            .ok_or_else(|| "Invalid password rule")?;

        let numbers = captures
            .iter()
            .skip(1)
            .take(2)
            .map(|capture| {
                capture
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| format!("Invalid number in rule: {}", err))
                    .and_then(|number| {
                        if number > 0 {
                            Ok(number)
                        } else {
                            Err("Rule numbers must be greater than zero"
                                .to_string())
                        }
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let first_num = numbers[0];
        let second_num = numbers[1];

        let character = captures
            .name("ch")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .ok_or_else(|| "Invalid rule character")?;

        Ok(Self {
            first_num,
            second_num,
            character,
        })
    }
}

impl FromStr for PasswordEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"(?P<rule>.*):\s(?P<pwd>.*)$")
            .unwrap()
            .captures(s)
            .ok_or_else(|| "Invalid password entry")?;
        let rule = captures.name("rule").unwrap().as_str().parse()?;
        let password = captures.name("pwd").unwrap().as_str().to_string();

        Ok(Self { rule, password })
    }
}
