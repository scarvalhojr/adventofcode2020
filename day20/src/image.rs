use super::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct Image {
    pub x_dimension: u32,
    pub y_dimension: u32,
    pub on_pixels: HashSet<Position>,
}

impl Image {
    pub fn count_on_pixels(&self) -> usize {
        self.on_pixels.len()
    }

    pub fn from(tiles: &[Tile]) -> Result<Self, String> {
        let tile_mult = (tiles.len() as f64).sqrt() as u32;
        if tile_mult <= 1 || tile_mult * tile_mult != tiles.len() as u32 {
            return Err("Image requires a square number of tiles".to_string());
        }

        let tile_dim = tiles.get(0).unwrap().image.x_dimension;
        let x_dimension = tile_dim * tile_mult;
        let y_dimension = x_dimension;
        let on_pixels = HashSet::new();

        let mut image = Self {
            x_dimension,
            y_dimension,
            on_pixels,
        };

        let mut remaining_tiles: HashMap<u32, &Tile> =
            tiles.iter().map(|tile| (tile.id, tile)).collect();

        let first_tile = remaining_tiles
            .values()
            .find_map(|tile| tile.align_to_north_west_edge(tiles))
            .ok_or_else(|| "Failed to find starting tile".to_string())?;
        image.copy_on_pixels(&first_tile.image, 0, 0);
        remaining_tiles.remove(&first_tile.id);
        let mut prev_row_border = first_tile.south;
        let mut last_border = first_tile.east;

        for row in 0..tile_mult {
            for col in 1..tile_mult {
                let next_tile = remaining_tiles
                    .values()
                    .find_map(|tile| tile.align(Direction::West, &last_border))
                    .ok_or_else(|| "Failed to find next tile".to_string())?;
                image.copy_on_pixels(
                    &next_tile.image,
                    tile_dim * col,
                    tile_dim * row,
                );
                remaining_tiles.remove(&next_tile.id);
                last_border = next_tile.east
            }
            if row < tile_mult - 1 {
                let tile = remaining_tiles
                    .values()
                    .find_map(|tile| {
                        tile.align(Direction::North, &prev_row_border)
                    })
                    .ok_or_else(|| "Failed to find next tile".to_string())?;
                image.copy_on_pixels(&tile.image, 0, tile_dim * (row + 1));
                remaining_tiles.remove(&tile.id);
                prev_row_border = tile.south;
                last_border = tile.east;
            }
        }

        Ok(image)
    }

    pub fn count_occurrences(&self, other: &Image) -> usize {
        (0..=self.y_dimension - other.y_dimension)
            .map(|y_shift| {
                (0..=self.x_dimension - other.x_dimension)
                    .map(move |x_shift| {
                        other.on_pixels.iter().all(|pos| {
                            self.on_pixels.contains(&Position::new(
                                pos.x_coord + x_shift,
                                pos.y_coord + y_shift,
                            ))
                        })
                    })
                    .filter(|&matches| matches)
                    .count()
            })
            .sum()
    }

    pub fn rotate_left(&self) -> Self {
        let on_pixels = self
            .on_pixels
            .iter()
            .map(|pos| {
                Position::new(pos.y_coord, self.x_dimension - 1 - pos.x_coord)
            })
            .collect();
        Self {
            x_dimension: self.y_dimension,
            y_dimension: self.x_dimension,
            on_pixels,
        }
    }

    pub fn rotate_right(&self) -> Self {
        let on_pixels = self
            .on_pixels
            .iter()
            .map(|pos| {
                Position::new(self.y_dimension - 1 - pos.y_coord, pos.x_coord)
            })
            .collect();
        Self {
            x_dimension: self.y_dimension,
            y_dimension: self.x_dimension,
            on_pixels,
        }
    }

    pub fn flip_horizontal(&self) -> Self {
        let on_pixels = self
            .on_pixels
            .iter()
            .map(|pos| {
                Position::new(pos.x_coord, self.y_dimension - 1 - pos.y_coord)
            })
            .collect();
        Self {
            x_dimension: self.x_dimension,
            y_dimension: self.y_dimension,
            on_pixels,
        }
    }

    pub fn flip_vertical(&self) -> Self {
        let on_pixels = self
            .on_pixels
            .iter()
            .map(|pos| {
                Position::new(self.x_dimension - 1 - pos.x_coord, pos.y_coord)
            })
            .collect();
        Self {
            x_dimension: self.x_dimension,
            y_dimension: self.y_dimension,
            on_pixels,
        }
    }

    pub fn strip_borders(&self) -> Self {
        if self.x_dimension < 3 || self.y_dimension < 3 {
            panic!("Image too small to strip borders");
        }
        let on_pixels = self
            .on_pixels
            .iter()
            .filter(|pos| {
                pos.x_coord > 0
                    && pos.y_coord > 0
                    && pos.x_coord < self.x_dimension - 1
                    && pos.y_coord < self.y_dimension - 1
            })
            .map(|pos| Position::new(pos.x_coord - 1, pos.y_coord - 1))
            .collect();
        Self {
            x_dimension: self.x_dimension - 2,
            y_dimension: self.y_dimension - 2,
            on_pixels,
        }
    }

    fn copy_on_pixels(&mut self, image: &Image, x_shift: u32, y_shift: u32) {
        self.on_pixels.extend(image.on_pixels.iter().map(|pos| {
            Position::new(pos.x_coord + x_shift, pos.y_coord + y_shift)
        }));
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y_coord in 0..self.y_dimension {
            for x_coord in 0..self.x_dimension {
                if self.on_pixels.contains(&Position::new(x_coord, y_coord)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_dimension = 0;
        let y_dimension = s.lines().count();
        let on_pixels = s
            .lines()
            .zip(0..)
            .flat_map(|(line, y_coord)| {
                x_dimension = x_dimension.max(line.chars().count());
                line.chars().zip(0..).filter(|(ch, _)| *ch != '.').map(
                    move |(ch, x_coord)| match ch {
                        '#' => Ok(Position::new(x_coord, y_coord)),
                        _ => {
                            Err(format!("Invalid character in image '{}'", ch))
                        }
                    },
                )
            })
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(Self {
            x_dimension: x_dimension.try_into().unwrap(),
            y_dimension: y_dimension.try_into().unwrap(),
            on_pixels,
        })
    }
}
