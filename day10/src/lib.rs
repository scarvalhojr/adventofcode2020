use std::collections::HashMap;
use std::convert::TryFrom;

pub fn part1(adapters: &[i32]) -> Option<i32> {
    let start = 0;
    let target = 3 + *adapters.iter().max()?;

    let mut sorted_adapters = adapters.to_vec();
    sorted_adapters.push(start);
    sorted_adapters.push(target);
    sorted_adapters.sort_unstable();

    let (count1, count3) = sorted_adapters[..]
        .windows(2)
        .flat_map(<&[i32; 2]>::try_from)
        .map(|[jolt1, jolt2]| jolt2 - jolt1)
        .fold((0, 0), |(count1, count3), diff| match diff {
            1 => (count1 + 1, count3),
            3 => (count1, count3 + 1),
            _ => (count1, count3),
        });
    Some(count1 * count3)
}

pub fn part2(adapters: &[i32]) -> Option<u64> {
    let start = 0;
    let target = 3 + *adapters.iter().max()?;

    let mut sorted_adapters = adapters.to_vec();
    sorted_adapters.push(start);
    sorted_adapters.push(target);
    sorted_adapters.sort_unstable();

    let mut path_count = HashMap::new();
    path_count.insert(start, 1);

    let len = 1 + adapters.len();
    for (index, adapter) in sorted_adapters.iter().enumerate().take(len) {
        let count = path_count.remove(&adapter)?;
        for next_adapter in sorted_adapters[index + 1..]
            .iter()
            .take_while(|&next| next - adapter <= 3)
        {
            path_count
                .entry(*next_adapter)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    path_count.get(&target).copied()
}
