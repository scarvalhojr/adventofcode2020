use super::*;
use regex::Regex;
use std::str::FromStr;

const MAX_DIMENSION: u32 = 32;

#[derive(Clone, Copy)]
pub struct Border {
    forward: u32,
    backward: u32,
}

impl Border {
    fn from<I>(on_pixels: I) -> Self
    where
        I: ExactSizeIterator<Item = bool>,
    {
        let mut forward = 0;
        let mut backward = 0;
        let mut forward_mask = 1 << (on_pixels.len() - 1);
        let mut backward_mask = 1;
        for on_pixel in on_pixels.into_iter() {
            if on_pixel {
                forward |= forward_mask;
                backward |= backward_mask;
            }
            forward_mask >>= 1;
            backward_mask <<= 1;
        }
        Border { forward, backward }
    }

    fn aligns(&self, other: &Border) -> bool {
        self.forward == other.backward
    }

    fn aligns_with_flip(&self, other: &Border) -> bool {
        self.forward == other.forward
    }

    fn can_align(&self, other: &Border) -> bool {
        self.aligns(other) || self.aligns_with_flip(other)
    }

    fn flip(&self) -> Self {
        Self {
            forward: self.backward,
            backward: self.forward,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pub id: u32,
    pub image: Image,
    pub north: Border,
    pub east: Border,
    pub south: Border,
    pub west: Border,
}

impl Tile {
    fn can_align(&self, border: &Border) -> bool {
        self.north.can_align(border)
            || self.east.can_align(border)
            || self.south.can_align(border)
            || self.west.can_align(border)
    }

    fn rotate_left(&self) -> Self {
        Self {
            id: self.id,
            image: self.image.rotate_left(),
            north: self.east,
            east: self.south,
            south: self.west,
            west: self.north,
        }
    }

    fn rotate_right(&self) -> Self {
        Self {
            id: self.id,
            image: self.image.rotate_right(),
            north: self.west,
            east: self.north,
            south: self.east,
            west: self.south,
        }
    }

    fn flip_horizontal(&self) -> Self {
        Self {
            id: self.id,
            image: self.image.flip_horizontal(),
            north: self.south.flip(),
            east: self.east.flip(),
            south: self.north.flip(),
            west: self.west.flip(),
        }
    }

    fn flip_vertical(&self) -> Self {
        Self {
            id: self.id,
            image: self.image.flip_vertical(),
            north: self.north.flip(),
            east: self.west.flip(),
            south: self.south.flip(),
            west: self.east.flip(),
        }
    }

    pub fn count_matching_borders(&self, tiles: &[Tile]) -> usize {
        [&self.north, &self.east, &self.south, &self.west]
            .iter()
            .filter(|border| {
                tiles
                    .iter()
                    .filter(|&tile| tile.id != self.id)
                    .any(|tile| tile.can_align(border))
            })
            .count()
    }

    pub fn align_to_north_west_edge(&self, tiles: &[Tile]) -> Option<Self> {
        match [&self.north, &self.south, &self.west, &self.east]
            .iter()
            .map(|border| {
                tiles
                    .iter()
                    .filter(|&tile| tile.id != self.id)
                    .any(|tile| tile.can_align(border))
            })
            .collect::<Vec<bool>>()
            .as_slice()
            .get(0..4)
        {
            Some(&[false, true, false, true]) => Some(self.clone()),
            Some(&[false, true, true, false]) => Some(self.flip_vertical()),
            Some(&[true, false, false, true]) => Some(self.flip_horizontal()),
            Some(&[true, false, true, false]) => {
                Some(self.flip_horizontal().flip_vertical())
            }
            _ => None,
        }
    }

    fn border_align(&self, other_border: &Border) -> Option<(Direction, bool)> {
        [
            (&self.north, Direction::North),
            (&self.east, Direction::East),
            (&self.south, Direction::South),
            (&self.west, Direction::West),
        ]
        .iter()
        .find_map(|(border, direction)| {
            if border.aligns(other_border) {
                Some((*direction, false))
            } else if border.aligns_with_flip(other_border) {
                Some((*direction, true))
            } else {
                None
            }
        })
    }

    pub fn align(&self, direction: Direction, border: &Border) -> Option<Self> {
        let (align_dir, flip) = self.border_align(&border)?;
        let rotated = match align_dir.rotate(direction) {
            Rotation::Nil => self.clone(),
            Rotation::Left => self.rotate_left(),
            Rotation::Right => self.rotate_right(),
            Rotation::Opposite => self.rotate_right().rotate_right(),
        };
        let aligned = match (flip, direction) {
            (true, Direction::North) => rotated.flip_vertical(),
            (true, Direction::South) => rotated.flip_vertical(),
            (true, Direction::West) => rotated.flip_horizontal(),
            (true, Direction::East) => rotated.flip_horizontal(),
            (false, _) => rotated,
        };
        Some(aligned)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^Tile (?P<id>\d+):\n(?P<pixels>[\.#\n\r]+)$")
                    .unwrap();
        }
        let captures = REGEX.captures(s).ok_or_else(|| "Invalid tile")?;
        let id = captures
            .name("id")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|err| format!("Invalid tile ID: {}", err))?;
        let mut image: Image =
            captures.name("pixels").unwrap().as_str().parse()?;

        if image.x_dimension != image.y_dimension {
            return Err(format!("Tile {} is not square", id));
        }
        if image.x_dimension < 3 {
            return Err(format!("Tle {} is too small", id));
        }
        if image.x_dimension > MAX_DIMENSION {
            return Err(format!(
                "Tile {} exceeds maximum dimension of {}",
                id, MAX_DIMENSION
            ));
        }

        let dimension = image.x_dimension;
        let north = Border::from((0..dimension).map(|x_coord| {
            image.on_pixels.contains(&Position::new(x_coord, 0))
        }));
        let east = Border::from((0..dimension).map(|y_coord| {
            image
                .on_pixels
                .contains(&Position::new(dimension - 1, y_coord))
        }));
        let south = Border::from((0..dimension).rev().map(|x_coord| {
            image
                .on_pixels
                .contains(&Position::new(x_coord, dimension - 1))
        }));
        let west = Border::from((0..dimension).rev().map(|y_coord| {
            image.on_pixels.contains(&Position::new(0, y_coord))
        }));

        image = image.strip_borders();

        Ok(Self {
            id,
            image,
            north,
            east,
            south,
            west,
        })
    }
}
