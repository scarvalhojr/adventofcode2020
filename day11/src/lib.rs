use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq)]
enum Area {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn adjacents(&self) -> impl Iterator<Item = Position> + '_ {
        all_directions().map(move |(row_delta, col_delta)| {
            Position::new(self.row + row_delta, self.col + col_delta)
        })
    }
}

fn all_directions() -> impl Iterator<Item = (i32, i32)> {
    (-1_i32..=1_i32)
        .flat_map(|row_delta: i32| {
            (-1_i32..=1_i32).map(move |col_delta: i32| (row_delta, col_delta))
        })
        .filter(|(row_delta, col_delta)| *row_delta != 0 || *col_delta != 0)
}

pub struct SeatingArea {
    grid: HashMap<Position, Area>,
}

impl FromStr for SeatingArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .zip(0_i32..)
            .flat_map(|(line, col)| {
                line.chars().zip(0_i32..).map(move |(ch, row)| match ch {
                    '.' => Ok((Position::new(row, col), Area::Floor)),
                    'L' => Ok((Position::new(row, col), Area::Empty)),
                    '#' => Ok((Position::new(row, col), Area::Occupied)),
                    _ => Err(format!("Invalid character in map: {}", ch)),
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self { grid })
    }
}
