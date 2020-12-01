use std::collections::HashSet;

pub fn part1(input: &[i32]) -> Option<i32> {
    let target = 2020;
    let items = input.iter().collect::<HashSet<_>>();

    // All entries in the input are unique
    items.iter().find_map(|&&item1| {
        items.get(&(target - item1)).map(|&&item2| item1 * item2)
    })
}

pub fn part2(input: &[i32]) -> Option<i32> {
    let target = 2020;
    let items = input.iter().collect::<HashSet<_>>();

    // All entries in the input are unique
    items.iter().find_map(|&&item1| {
        items.iter().find_map(|&&item2| {
            items
                .get(&(target - item1 - item2))
                .map(|&&item3| item1 * item2 * item3)
        })
    })
}
