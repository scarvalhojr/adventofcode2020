use std::collections::HashSet;
use std::str::FromStr;

type Position = (usize, usize);
type Slope = (usize, usize);

pub struct TreeMap {
    width: usize,
    height: usize,
    trees: HashSet<Position>,
}

impl TreeMap {
    fn count_hits(&self, slope: &Slope) -> usize {
        let mut position = Position::default();
        let mut hits = 0;

        while position.1 < self.height {
            if self.trees.contains(&position) {
                hits += 1;
            }
            position = self.slide(&position, slope);
        }
        hits
    }

    fn slide(&self, position: &Position, slope: &Slope) -> Position {
        ((position.0 + slope.0) % self.width, position.1 + slope.1)
    }
}

pub fn part1(treemap: &TreeMap) -> usize {
    treemap.count_hits(&(3, 1))
}

pub fn part2(treemap: &TreeMap) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| treemap.count_hits(slope))
        .product()
}

impl FromStr for TreeMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .enumerate()
            .flat_map(|(y_coord, line)| {
                line.chars().enumerate().filter(|(_, ch)| *ch != '.').map(
                    move |(x_coord, ch)| match ch {
                        '#' => Ok((x_coord, y_coord)),
                        _ => Err(format!("Invalid character in map: {}", ch)),
                    },
                )
            })
            .collect::<Result<HashSet<_>, _>>()?;
        let width = s.lines().map(|line| line.len()).max().unwrap_or(0);
        let height = s.lines().count();
        Ok(Self {
            width,
            height,
            trees,
        })
    }
}
