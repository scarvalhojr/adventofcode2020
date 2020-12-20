use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<usize, String> {
    let mut grid: Grid<Cube3D> = input.parse()?;
    for _cycle in 1..=6 {
        grid = grid.update();
    }
    Ok(grid.count_active())
}

pub fn part2(input: &str) -> Result<usize, String> {
    let mut grid: Grid<Cube4D> = input.parse()?;
    for _cycle in 1..=6 {
        grid = grid.update();
    }
    Ok(grid.count_active())
}

trait Cube {
    fn neighbours(&self) -> Box<dyn Iterator<Item = Self> + '_>;
    fn from_2d(coord0: i32, coord1: i32) -> Self;
}

struct Grid<T> {
    active: HashSet<T>,
}

impl<T: Cube + Clone + Hash + Eq> Grid<T> {
    fn count_active(&self) -> usize {
        self.active.len()
    }

    fn update(&self) -> Self {
        let mut active = HashSet::new();
        let mut inactive: HashMap<T, usize> = HashMap::new();

        for cube in self.active.iter() {
            let mut active_neighbours = 0;
            for neighbour in cube.neighbours() {
                if self.active.contains(&neighbour) {
                    active_neighbours += 1;
                } else {
                    inactive
                        .entry(neighbour)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            if active_neighbours == 2 || active_neighbours == 3 {
                active.insert(cube.clone());
            }
        }

        active.extend(
            inactive
                .drain()
                .filter(|(_, active_neighbours)| *active_neighbours == 3)
                .map(|(cube, _)| cube),
        );

        Self { active }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Cube3D(i32, i32, i32);

impl Cube for Cube3D {
    fn neighbours(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(
            (-1..=1)
                .flat_map(|delta0| {
                    (-1..=1).flat_map(move |delta1| {
                        (-1..=1).map(move |delta2| (delta0, delta1, delta2))
                    })
                })
                .filter(|(delta0, delta1, delta2)| {
                    *delta0 != 0 || *delta1 != 0 || *delta2 != 0
                })
                .map(move |(delta0, delta1, delta2)| {
                    Self(self.0 + delta0, self.1 + delta1, self.2 + delta2)
                }),
        )
    }

    fn from_2d(coord0: i32, coord1: i32) -> Self {
        Self(coord0, coord1, 0)
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Cube4D(i32, i32, i32, i32);

impl Cube for Cube4D {
    fn neighbours(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(
            (-1..=1)
                .flat_map(|delta0| {
                    (-1..=1).flat_map(move |delta1| {
                        (-1..=1).flat_map(move |delta2| {
                            (-1..=1).map(move |delta3| {
                                (delta0, delta1, delta2, delta3)
                            })
                        })
                    })
                })
                .filter(|(delta0, delta1, delta2, delta3)| {
                    *delta0 != 0 || *delta1 != 0 || *delta2 != 0 || *delta3 != 0
                })
                .map(move |(delta0, delta1, delta2, delta3)| {
                    Self(
                        self.0 + delta0,
                        self.1 + delta1,
                        self.2 + delta2,
                        self.3 + delta3,
                    )
                }),
        )
    }

    fn from_2d(coord0: i32, coord1: i32) -> Self {
        Self(coord0, coord1, 0, 0)
    }
}

impl<T: Cube + Hash + Eq> FromStr for Grid<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active = s
            .lines()
            .zip(0..)
            .flat_map(|(line, coord0)| {
                line.chars().zip(0..).filter(|&(ch, _)| ch != '.').map(
                    move |(ch, coord1)| match ch {
                        '#' => Ok(T::from_2d(coord0, coord1)),
                        _ => Err(format!("Invalid character in grid: {}", ch)),
                    },
                )
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { active })
    }
}
