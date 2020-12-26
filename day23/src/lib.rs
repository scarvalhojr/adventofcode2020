use std::str::FromStr;

#[derive(Clone)]
pub struct Cups {
    max: usize,
    current: usize,
    next: Vec<usize>,
}

impl Cups {
    fn do_move(&mut self) {
        let pick1 = self.next[self.current];
        let pick2 = self.next[pick1];
        let pick3 = self.next[pick2];

        let mut dest = self.decrement(self.current);
        while dest == pick1 || dest == pick2 || dest == pick3 {
            dest = self.decrement(dest)
        }

        self.next[self.current] = self.next[pick3];
        self.next[pick3] = self.next[dest];
        self.next[dest] = pick1;
        self.current = self.next[self.current];
    }

    fn decrement(&self, number: usize) -> usize {
        if number == 1 {
            self.max
        } else {
            number - 1
        }
    }

    fn get_labels(&self) -> String {
        let mut labels = Vec::new();
        let mut cup = self.next[1];
        while cup != 1 {
            labels.push(cup.to_string());
            cup = self.next[cup];
        }
        labels.concat()
    }

    fn extend(&self, length: usize) -> Self {
        if length < self.next.len() {
            panic!("New length must be greater than current length");
        }

        let mut next = self.next.clone();
        let mut next_elem = self.max + 1;
        next.resize_with(length + 1, || {
            next_elem += 1;
            next_elem
        });
        next[length] = self.current;

        let mut last = self.next[self.current];
        while self.next[last] != self.current {
            last = self.next[last];
        }
        next[last] = self.max + 1;

        Self {
            max: length,
            current: self.current,
            next,
        }
    }
}

pub fn part1(cups: &Cups) -> String {
    let mut cups = cups.clone();
    for _ in 1..=100 {
        cups.do_move();
    }
    cups.get_labels()
}

pub fn part2(cups: &Cups) -> usize {
    let mut cups = cups.extend(1_000_000);
    for _ in 1..=10_000_000 {
        cups.do_move();
    }
    let cup1 = cups.next[1];
    let cup2 = cups.next[cup1];
    cup1 * cup2
}

impl FromStr for Cups {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut labels = s
            .trim()
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .map(|digit| digit as usize)
                    .ok_or_else(|| format!("Invalid label for cup '{}'", ch))
            })
            .collect::<Result<Vec<usize>, String>>()?;

        // Assuming there are no gaps, no repetitions, no zeros
        let max = *labels.iter().max().ok_or_else(|| "No label found")?;
        let current = labels[0];
        let mut next = vec![0; labels.len() + 1];
        for window in labels.windows(2) {
            match *window {
                [num1, num2] => next[num1] = num2,
                _ => unreachable!(),
            }
        }
        next[labels.pop().unwrap()] = current;
        Ok(Self { max, current, next })
    }
}
