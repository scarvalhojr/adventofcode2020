#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

pub type Ticket = Vec<u64>;

#[derive(Debug)]
pub struct Rule {
    name: String,
    range1: (u64, u64),
    range2: (u64, u64),
}

impl Rule {
    fn accepts(&self, value: u64) -> bool {
        (value >= self.range1.0 && value <= self.range1.1)
            || (value >= self.range2.0 && value <= self.range2.1)
    }

    fn accepts_all<'a, I>(&self, values: I) -> bool
    where
        I: IntoIterator<Item = &'a u64>,
    {
        values.into_iter().all(|value| self.accepts(*value))
    }
}

fn ticket_error_rate(ticket: &[u64], rules: &[Rule]) -> u64 {
    ticket
        .iter()
        .filter(|&value| rules.iter().all(|rule| !rule.accepts(*value)))
        .sum()
}

fn valid_ticket(ticket: &[u64], rules: &[Rule]) -> bool {
    ticket
        .iter()
        .all(|&value| rules.iter().any(|rule| rule.accepts(value)))
}

pub fn part1(rules: &[Rule], nearby_tickets: &[Ticket]) -> u64 {
    nearby_tickets
        .iter()
        .map(|ticket| ticket_error_rate(ticket, rules))
        .sum()
}

fn match_rules(
    rules: &[Rule],
    tickets: &[Ticket],
) -> Option<HashMap<usize, usize>> {
    let num_fields = rules.len();

    // Collect all possible values for each field - based on valid tickets only
    let mut field_values: Vec<HashSet<u64>> = vec![HashSet::new(); num_fields];
    for ticket in tickets {
        if valid_ticket(ticket, rules) {
            for (index, value) in ticket.iter().enumerate().take(num_fields) {
                field_values[index].insert(*value);
            }
        }
    }

    // For each rule, find a set of fields that it could match
    let mut possible_matches: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .enumerate()
        .map(|(rule_index, rule)| {
            let possible_fields = field_values
                .iter()
                .enumerate()
                .filter(|(_, values)| rule.accepts_all(*values))
                .map(|(field_index, _)| field_index)
                .collect();
            (rule_index, possible_fields)
        })
        .collect();

    let mut rule_match: HashMap<usize, usize> = HashMap::new();

    // By elimination, assign fields to rules that can only apply to 1 field
    while let Some((&rule_index, _)) = possible_matches
        .iter()
        .find(|(_, possible_fields)| possible_fields.len() == 1)
    {
        let field_index = possible_matches
            .remove(&rule_index)
            .unwrap()
            .drain()
            .next()
            .unwrap();

        // Remove the assigned field from other possible matches
        for (_, possible_fields) in possible_matches.iter_mut() {
            possible_fields.remove(&field_index);
        }

        // Assign field to rule
        rule_match.insert(rule_index, field_index);
    }

    if possible_matches.is_empty() {
        Some(rule_match)
    } else {
        // Not all rules could be matched
        None
    }
}

pub fn part2(
    rules: &[Rule],
    your_ticket: &[u64],
    nearby_tickets: &[Ticket],
) -> Option<u64> {
    let rule_to_field = match_rules(rules, nearby_tickets)?;

    rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(rule_index, _)| {
            your_ticket.get(*rule_to_field.get(&rule_index).unwrap())
        })
        .try_fold(1, |product, value| value.map(|val| val * product))
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(concat!(
                r"^(?P<name>.*): (?P<start1>\d+)-(?P<end1>\d+) or ",
                r"(?P<start2>\d+)-(?P<end2>\d+)$",
            ))
            .unwrap();
        }

        let captures = REGEX.captures(s).ok_or_else(|| "Invalid rule")?;
        let name = captures.name("name").unwrap().as_str().to_string();
        let numbers = ["start1", "end1", "start2", "end2"]
            .iter()
            .map(|field| {
                captures
                    .name(field)
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| format!("Invalid number: {}", err))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let range1 = (numbers[0], numbers[1]);
        let range2 = (numbers[2], numbers[3]);

        Ok(Self {
            name,
            range1,
            range2,
        })
    }
}

pub fn parse_ticket(line: &str) -> Result<Ticket, String> {
    line.split(',')
        .map(|value| {
            value.trim().parse().map_err(|err: ParseIntError| {
                format!("Invalid ticket value '{}': {}", value, err)
            })
        })
        .collect::<Result<Vec<_>, _>>()
}
