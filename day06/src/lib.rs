use std::collections::HashSet;

pub type Group = Vec<Answer>;
pub type Answer = HashSet<char>;

pub fn part1(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(HashSet::new(), |all, answer| {
                    all.union(answer).cloned().collect()
                })
                .len()
        })
        .sum()
}

pub fn part2(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            // fold_first would be perfect here (available in nightly only)
            let mut answers = group.iter();
            let first = answers.next().cloned().unwrap_or_default();
            answers
                .fold(first, |all, answer| {
                    all.intersection(answer).cloned().collect()
                })
                .len()
        })
        .sum()
}
