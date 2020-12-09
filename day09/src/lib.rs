pub fn part1(input: &[i64], preamble_len: usize) -> Option<i64> {
    input.windows(preamble_len + 1).find_map(|window| {
        let next_number = window[preamble_len];
        let has_sum = window.iter().enumerate().take(preamble_len - 1).any(
            |(index, number)| {
                let target = next_number - number;
                window[index + 1..preamble_len].iter().any(|&n| n == target)
            },
        );

        if !has_sum {
            Some(next_number)
        } else {
            None
        }
    })
}

pub fn part2(input: &[i64], target: i64) -> Option<i64> {
    input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .find_map(|(start_pos, number)| {
            let mut end_pos = start_pos + 1;
            let mut sum = number + input[end_pos];
            while end_pos < input.len() && sum < target {
                end_pos += 1;
                sum += input[end_pos];
            }
            if sum == target {
                Some((start_pos, end_pos))
            } else {
                None
            }
        })
        .map(|(start_pos, end_pos)| {
            let min = *input[start_pos..=end_pos].iter().min().unwrap();
            let max = *input[start_pos..=end_pos].iter().max().unwrap();
            min + max
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let preamble_len = 25;
        let mut numbers: Vec<_> = (1..=25).collect();

        numbers.push(26);
        assert_eq!(part1(&numbers, preamble_len), None);
        numbers.pop();

        numbers.push(49);
        assert_eq!(part1(&numbers, preamble_len), None);
        numbers.pop();

        numbers.push(100);
        assert_eq!(part1(&numbers, preamble_len), Some(100));
        numbers.pop();

        numbers.push(50);
        assert_eq!(part1(&numbers, preamble_len), Some(50));
        numbers.pop();
    }

    #[test]
    fn part1_example2() {
        let preamble_len = 5;
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127,
            219, 299, 277, 309, 576,
        ];
        assert_eq!(part1(&numbers, preamble_len), Some(127));
    }
}
