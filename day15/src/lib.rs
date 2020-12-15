use std::collections::HashMap;
use std::convert::TryInto;

fn find_nth_number(starting_numbers: &[i32], nth: i32) -> Option<i32> {
    // Assumming there's no repetition in starting numbers!
    let mut last_seen: HashMap<i32, i32> =
        starting_numbers.iter().copied().zip(1..).collect();

    let mut turn = starting_numbers.len().try_into().ok()?;
    let mut number = *starting_numbers.iter().last()?;
    while turn < nth {
        number = match last_seen.insert(number, turn) {
            Some(last_turn) => turn - last_turn,
            _ => 0,
        };
        turn += 1;
    }
    Some(number)
}

pub fn part1(numbers: &[i32]) -> Option<i32> {
    find_nth_number(numbers, 2020)
}

pub fn part2(numbers: &[i32]) -> Option<i32> {
    find_nth_number(numbers, 30_000_000)
}
