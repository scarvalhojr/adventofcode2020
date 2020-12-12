use std::convert::TryFrom;
use std::str::FromStr;

pub mod part1;
pub mod part2;

pub enum Action {
    Direction(Direction, i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn degrees(&self) -> i32 {
        match self {
            Self::North => 0,
            Self::East => 90,
            Self::South => 180,
            Self::West => 270,
        }
    }

    fn turn(&self, degrees: i32) -> Result<Self, &'static str> {
        Self::try_from(self.degrees() + degrees)
    }
}

impl TryFrom<i32> for Direction {
    type Error = &'static str;

    fn try_from(degrees: i32) -> Result<Self, Self::Error> {
        match degrees.rem_euclid(360) {
            0 => Ok(Self::North),
            90 => Ok(Self::East),
            180 => Ok(Self::South),
            270 => Ok(Self::West),
            _ => Err("Direction requires a multiple of 90 degrees"),
        }
    }
}

#[derive(Default)]
struct Coordinates {
    vertical: i32,
    horizontal: i32,
}

impl Coordinates {
    fn new(vertical: i32, horizontal: i32) -> Self {
        Self {
            vertical,
            horizontal,
        }
    }

    fn move_direction(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::North => self.vertical += distance,
            Direction::East => self.horizontal += distance,
            Direction::South => self.vertical -= distance,
            Direction::West => self.horizontal -= distance,
        }
    }

    fn move_distance(&mut self, vertical_dist: i32, horizontal_dist: i32) {
        self.vertical += vertical_dist;
        self.horizontal += horizontal_dist;
    }

    fn rotate(&mut self, degrees: i32) {
        match degrees.rem_euclid(360) {
            0 => (),
            90 => {
                let vertical = self.vertical;
                self.vertical = -self.horizontal;
                self.horizontal = vertical;
            }
            180 => {
                self.vertical = -self.vertical;
                self.horizontal = -self.horizontal;
            }
            270 => {
                let vertical = self.vertical;
                self.vertical = self.horizontal;
                self.horizontal = -vertical;
            }
            _ => panic!(
                "Coordinates can only be rotataed in multiples of 90 degrees"
            ),
        }
    }

    fn distance_to_origin(&self) -> i32 {
        self.vertical.abs() + self.horizontal.abs()
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s.get(0..1).ok_or_else(|| "Invalid action")?;
        let value = s
            .get(1..)
            .ok_or_else(|| "Invalid action")?
            .parse()
            .map_err(|err| format!("Invalid action value: {}", err))?;
        match action {
            "N" => Ok(Self::Direction(Direction::North, value)),
            "E" => Ok(Self::Direction(Direction::East, value)),
            "S" => Ok(Self::Direction(Direction::South, value)),
            "W" => Ok(Self::Direction(Direction::West, value)),
            "F" => Ok(Self::Forward(value)),
            "L" if value.rem_euclid(90) == 0 => Ok(Self::Left(value)),
            "L" => Err(String::from("Left turn must be a multiple of 90")),
            "R" if value.rem_euclid(90) == 0 => Ok(Self::Right(value)),
            "R" => Err(String::from("Right turn must be a multiple of 90")),
            _ => Err(format!("Unknown action: {}", action)),
        }
    }
}
