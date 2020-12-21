#[macro_use]
extern crate lazy_static;

use regex::{Match, Regex};
use std::collections::HashMap;
use std::str::FromStr;

pub fn part1(rules: &[Rule], messages: &[String]) -> Result<usize, String> {
    PatternMatcher::new(rules).count_matches(messages)
}

pub fn part2(rules: &[Rule], messages: &[String]) -> Result<usize, String> {
    let mut matcher = PatternMatcher::new(rules);
    let pattern8 = Pattern::Alternative(vec![42], vec![42, 8]);
    let pattern11 = Pattern::Alternative(vec![42, 31], vec![42, 11, 31]);
    matcher.update_rule(8, &pattern8);
    matcher.update_rule(11, &pattern11);
    matcher.count_matches(messages)
}

pub type RuleNumber = u32;

pub struct Rule {
    number: RuleNumber,
    pattern: Pattern,
}

pub enum Pattern {
    Terminal(char),
    Sequence(Vec<RuleNumber>),
    Alternative(Vec<RuleNumber>, Vec<RuleNumber>),
}

struct PatternMatcher<'a> {
    rules: HashMap<RuleNumber, &'a Pattern>,
}

impl<'a> PatternMatcher<'a> {
    fn new(rule_list: &'a [Rule]) -> Self {
        let rules = rule_list
            .iter()
            .map(|rule| (rule.number, &rule.pattern))
            .collect();
        Self { rules }
    }

    fn update_rule(&mut self, rule_num: RuleNumber, pattern: &'a Pattern) {
        self.rules.insert(rule_num, pattern);
    }

    fn count_matches(&self, messages: &[String]) -> Result<usize, String> {
        let mut count = 0;
        for message in messages {
            if self.matches(message)? {
                count += 1;
            }
        }
        Ok(count)
    }

    fn matches(&self, message: &str) -> Result<bool, String> {
        let chars = message.chars().collect::<Vec<_>>();
        self.match_rule(0, &chars, &[0])
            .map_err(|num| format!("Missing rule {}", num))
            .map(|matches| {
                matches.iter().any(|&match_len| match_len == message.len())
            })
    }

    fn match_rule(
        &self,
        rule_num: RuleNumber,
        chars: &[char],
        positions: &[usize],
    ) -> Result<Vec<usize>, RuleNumber> {
        let mut matches = Vec::new();

        match self.rules.get(&rule_num).ok_or_else(|| rule_num)? {
            Pattern::Terminal(ch) => {
                for &position in positions {
                    if chars.get(position) == Some(ch) {
                        matches.push(position + 1);
                    }
                }
            }
            Pattern::Sequence(rules) => {
                for &position in positions {
                    matches.extend(
                        self.match_rule_seq(rules, chars, position)?.drain(..),
                    );
                }
            }
            Pattern::Alternative(rules1, rules2) => {
                for &position in positions {
                    matches.extend(
                        self.match_rule_seq(rules1, chars, position)?.drain(..),
                    );
                    matches.extend(
                        self.match_rule_seq(rules2, chars, position)?.drain(..),
                    );
                }
            }
        }

        Ok(matches)
    }

    fn match_rule_seq(
        &self,
        rules: &[RuleNumber],
        chars: &[char],
        position: usize,
    ) -> Result<Vec<usize>, RuleNumber> {
        let mut matches = vec![position];
        for &rule_num in rules {
            matches = self.match_rule(rule_num, chars, &matches)?;
        }
        Ok(matches)
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rule = s.split(": ");
        let number = rule
            .next()
            .ok_or_else(|| "Invalid rule.".to_string())?
            .parse()
            .map_err(|err| format!("Invalid rule number: {}", err))?;
        let pattern = rule
            .next()
            .ok_or_else(|| "Missing pattern.".to_string())?
            .parse()?;
        Ok(Rule { number, pattern })
    }
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(concat!(
                r#"^((?P<numbers>[\d ]+)|"#,
                r#"("(?P<char>.)")|"#,
                r#"((?P<numbers1>[\d ]+) \| (?P<numbers2>[\d ]+)))$"#,
            ))
            .unwrap();
        }

        let parse = |capture: Match| {
            capture
                .as_str()
                .split_whitespace()
                .map(|num| {
                    num.parse().map_err(|err| {
                        format!("Invalid rule number '{}': {}", num, err)
                    })
                })
                .collect::<Result<Vec<u32>, _>>()
        };

        let captures = REGEX.captures(s).ok_or_else(|| "Invalid pattern")?;
        if let Some(capture) = captures.name("numbers") {
            Ok(Self::Sequence(parse(capture)?))
        } else if let Some(capture) = captures.name("char") {
            let ch = capture.as_str().chars().next().unwrap();
            Ok(Self::Terminal(ch))
        } else {
            let numbers1 = captures.name("numbers1").unwrap();
            let numbers2 = captures.name("numbers2").unwrap();
            Ok(Self::Alternative(parse(numbers1)?, parse(numbers2)?))
        }
    }
}
