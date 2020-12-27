use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

pub type Instruction = Vec<Direction>;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct HexPosition {
    horizontal: i32,
    vertical: i32,
}

impl HexPosition {
    fn step(&self, direction: Direction) -> Self {
        let (horizontal_shift, vertical_shift) = match direction {
            Direction::East => (2, 0),
            Direction::SouthEast => (1, 1),
            Direction::NorthEast => (1, -1),
            Direction::West => (-2, 0),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, -1),
        };

        Self {
            horizontal: self.horizontal + horizontal_shift,
            vertical: self.vertical + vertical_shift,
        }
    }

    fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        [
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
        .iter()
        .map(move |direction| self.step(*direction))
    }
}

#[derive(Default)]
struct Grid {
    black_tiles: HashSet<HexPosition>,
}

impl Grid {
    fn flip_tile(&mut self, instruction: &[Direction]) {
        let mut position = HexPosition::default();
        for direction in instruction {
            position = position.step(*direction);
        }
        if !self.black_tiles.remove(&position) {
            self.black_tiles.insert(position);
        }
    }

    fn count_black(&self) -> usize {
        self.black_tiles.len()
    }

    fn update(&mut self) {
        let mut white_tiles: HashMap<HexPosition, usize> = HashMap::new();
        let remain_black = self
            .black_tiles
            .iter()
            .filter(|&tile| {
                let mut black_neighbours = 0;
                for neighbour in tile.neighbours() {
                    if self.black_tiles.contains(&neighbour) {
                        black_neighbours += 1;
                    } else {
                        white_tiles
                            .entry(neighbour)
                            .and_modify(|black_neighbours| {
                                *black_neighbours += 1
                            })
                            .or_insert(1);
                    }
                }
                black_neighbours == 1 || black_neighbours == 2
            })
            .cloned()
            .collect();
        self.black_tiles = remain_black;
        self.black_tiles.extend(
            white_tiles
                .drain()
                .filter(|(_, black_neighbours)| *black_neighbours == 2)
                .map(|(position, _)| position),
        );
    }
}

pub fn part1(instructions: &[Instruction]) -> usize {
    let mut grid = Grid::default();
    for instruction in instructions {
        grid.flip_tile(instruction);
    }
    grid.count_black()
}

pub fn part2(instructions: &[Instruction]) -> usize {
    let mut grid = Grid::default();
    for instruction in instructions {
        grid.flip_tile(instruction);
    }
    for _ in 1..=100 {
        grid.update();
    }
    grid.count_black()
}

pub fn parse_instruction(s: &str) -> Result<Instruction, String> {
    let mut instr = Vec::new();
    let mut chars = s.chars().peekable();
    while let (Some(ch), next) = (chars.next(), chars.peek()) {
        match ch {
            'e' => instr.push(Direction::East),
            'w' => instr.push(Direction::West),
            'n' if next == Some(&'e') => {
                chars.next();
                instr.push(Direction::NorthEast)
            }
            'n' if next == Some(&'w') => {
                chars.next();
                instr.push(Direction::NorthWest)
            }
            's' if next == Some(&'e') => {
                chars.next();
                instr.push(Direction::SouthEast)
            }
            's' if next == Some(&'w') => {
                chars.next();
                instr.push(Direction::SouthWest)
            }
            _ => return Err(format!("Invalid direction '{}'", ch)),
        }
    }
    Ok(instr)
}
