use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Grid4D {
    active: HashSet<Hypercube>,
}

impl Grid4D {
    fn count_active(&self) -> usize {
        self.active.len()
    }

    fn update(&self) -> Self {
        let mut next_active = HashSet::new();
        let mut inactive: HashMap<Hypercube, usize> = HashMap::new();

        for hypercube in self.active.iter() {
            let mut active_neighbours = 0;
            for neighbour in hypercube.neighbours() {
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
                next_active.insert(*hypercube);
            }
        }

        next_active.extend(
            inactive
                .drain()
                .filter(|(_, active_neighbours)| *active_neighbours == 3)
                .map(|(hypercube, _)| hypercube),
        );

        Self {
            active: next_active,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Hypercube {
    x_coord: i32,
    y_coord: i32,
    z_coord: i32,
    w_coord: i32,
}

impl Hypercube {
    fn new(x_coord: i32, y_coord: i32, z_coord: i32, w_coord: i32) -> Self {
        Self {
            x_coord,
            y_coord,
            z_coord,
            w_coord,
        }
    }

    fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        (-1..=1)
            .flat_map(|x_delta| {
                (-1..=1).flat_map(move |y_delta| {
                    (-1..=1).flat_map(move |z_delta| {
                        (-1..=1).map(move |w_delta| {
                            (x_delta, y_delta, z_delta, w_delta)
                        })
                    })
                })
            })
            .filter(|(x_delta, y_delta, z_delta, w_delta)| {
                *x_delta != 0 || *y_delta != 0 || *z_delta != 0 || *w_delta != 0
            })
            .map(move |(x_delta, y_delta, z_delta, w_delta)| {
                Self::new(
                    self.x_coord + x_delta,
                    self.y_coord + y_delta,
                    self.z_coord + z_delta,
                    self.w_coord + w_delta,
                )
            })
    }
}

pub fn part2(start: &Grid4D) -> usize {
    let mut grid = start.update();
    for _cycle in 1..6 {
        grid = grid.update();
    }
    grid.count_active()
}

impl FromStr for Grid4D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active = s
            .lines()
            .zip(0..)
            .flat_map(|(line, y_coord)| {
                line.chars().zip(0..).filter(|&(ch, _)| ch != '.').map(
                    move |(ch, x_coord)| match ch {
                        '#' => Ok(Hypercube::new(x_coord, y_coord, 0, 0)),
                        _ => Err(format!("Invalid character in grid: {}", ch)),
                    },
                )
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { active })
    }
}
