#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub struct Rule {
    outer: String,
    inner: Vec<(u32, String)>,
}

pub fn part1(rules: &[Rule]) -> usize {
    let mut is_inside: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rule in rules {
        for (_, inner_bag) in rule.inner.iter() {
            is_inside
                .entry(&inner_bag)
                .and_modify(|set| {
                    set.insert(&rule.outer);
                })
                .or_insert_with(|| {
                    vec![rule.outer.as_ref()].into_iter().collect()
                });
        }
    }

    let mut colours = HashSet::new();
    let mut pending: VecDeque<_> = vec!["shiny gold"].into_iter().collect();
    while let Some(next) = pending.pop_front() {
        if let Some(bags) = is_inside.get(next) {
            for bag in bags {
                if colours.insert(bag) {
                    pending.push_back(bag);
                }
            }
        }
    }
    colours.len()
}

pub fn part2(rules: &[Rule]) -> u32 {
    let mut total = 0;
    let mut bags: VecDeque<_> = vec![(1, "shiny gold")].into_iter().collect();
    while let Some((count, bag)) = bags.pop_front() {
        total += count;
        if let Some(rule) = rules.iter().find(|rule| rule.outer == bag) {
            for (inner_count, inner_bag) in rule.inner.iter() {
                bags.push_back((count * inner_count, inner_bag))
            }
        }
    }
    total - 1
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref OUTER_REGEX: Regex = Regex::new(
                r"^(?P<outer>.*) bags contain ((no other bags)|(?P<bags>.*))\.$"
            )
            .unwrap();
            static ref INNER_REGEX: Regex =
                Regex::new(r"^(?P<count>\d+) (?P<bag>.*) bags?$").unwrap();
        }

        let outer_captures =
            OUTER_REGEX.captures(s).ok_or_else(|| "Invalid rule")?;
        let outer = outer_captures.name("outer").unwrap().as_str().to_string();
        let inner = outer_captures
            .name("bags")
            .map(|capture| {
                capture
                    .as_str()
                    .split(", ")
                    .map(|bags| {
                        let inner_captures =
                            INNER_REGEX.captures(bags).ok_or_else(|| {
                                format!("Invalid rule contents: {}", bags)
                            })?;
                        let count = inner_captures
                            .name("count")
                            .unwrap()
                            .as_str()
                            .parse()
                            .map_err(|err| {
                                format!("Invalid number of bags: {}", err)
                            })?;
                        let bag = inner_captures
                            .name("bag")
                            .unwrap()
                            .as_str()
                            .to_string();
                        Ok((count, bag))
                    })
                    .collect::<Result<Vec<_>, Self::Err>>()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Self { outer, inner })
    }
}
