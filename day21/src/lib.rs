#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;

pub type Ingredient = String;
pub type Allergen = String;

#[derive(Clone, Debug)]
pub struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

pub fn part1(foods: &[Food]) -> Option<usize> {
    let mut possible: HashMap<&Allergen, HashSet<Ingredient>> = HashMap::new();
    for food in foods {
        for allergen in food.allergens.iter() {
            possible
                .entry(allergen)
                .and_modify(|list| {
                    list.retain(|ingr| food.ingredients.contains(ingr))
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    let bad_foods: HashSet<&Ingredient> = possible.values().flatten().collect();

    if possible.len() != bad_foods.len() {
        // Not enough information to rule out some ingredients
        return None;
    }

    let total = foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|&ingr| !bad_foods.contains(ingr))
                .count()
        })
        .sum();
    Some(total)
}

pub fn part2(foods: &[Food]) -> Option<String> {
    let mut possible: HashMap<&Allergen, HashSet<Ingredient>> = HashMap::new();
    for food in foods {
        for allergen in food.allergens.iter() {
            possible
                .entry(allergen)
                .and_modify(|list| {
                    list.retain(|ingr| food.ingredients.contains(ingr))
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    let mut matches: HashMap<Ingredient, &Allergen> = HashMap::new();
    loop {
        let mut done = true;
        for (allergen, list) in
            possible.iter_mut().filter(|(_, list)| list.len() == 1)
        {
            let ingredient = list.drain().next().unwrap();
            matches.insert(ingredient, allergen);
            done = false;
        }
        if done {
            break;
        }

        for list in possible.values_mut().filter(|list| list.len() > 1) {
            list.retain(|ingr| !matches.contains_key(ingr));
        }
        possible.retain(|_, list| !list.is_empty());
    }

    if !possible.is_empty() {
        // Not enough information to match allergens to ingredients
        return None;
    }

    let sorted: BTreeMap<&Allergen, &Ingredient> = matches
        .iter()
        .map(|(ingr, &allergen)| (allergen, ingr))
        .collect();
    let ingedient_list = sorted
        .values()
        .map(|&s| s.as_str())
        .collect::<Vec<&str>>()
        .join(",");
    Some(ingedient_list)
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"^(?P<ingredients>.*) \(contains (?P<allergens>.*)\)$",
            )
            .unwrap();
        }
        let captures = REGEX.captures(s).ok_or_else(|| "Invalid food")?;
        let ingredients = captures
            .name("ingredients")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|word| word.trim().to_string())
            .collect();
        let allergens = captures
            .name("allergens")
            .unwrap()
            .as_str()
            .split(',')
            .map(|word| word.trim().to_string())
            .collect();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}
