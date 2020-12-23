#[macro_use]
extern crate lazy_static;

pub mod image;
pub mod tile;

use image::Image;
use tile::Tile;

pub fn part1(tiles: &[Tile]) -> Option<u64> {
    let corner_tiles = tiles
        .iter()
        .filter(|tile| tile.count_matching_borders(tiles) == 2)
        .map(|tile| u64::from(tile.id))
        .collect::<Vec<_>>();
    if corner_tiles.len() == 4 {
        Some(corner_tiles.iter().product())
    } else {
        None
    }
}

pub fn part2(tiles: &[Tile]) -> Result<usize, String> {
    let image = Image::from(tiles)?;
    let mut pattern: Image = concat!(
        "..................#.\n",
        "#....##....##....###\n",
        ".#..#..#..#..#..#...\n",
    )
    .parse()?;

    let transformations = [
        Image::rotate_left,
        Image::rotate_left,
        Image::rotate_left,
        Image::flip_horizontal,
        Image::rotate_left,
        Image::rotate_left,
        Image::rotate_left,
    ];

    let mut occurrences = image.count_occurrences(&pattern);
    for transform in &transformations {
        // Assumming the pattern only occurs in the image in one orientation
        if occurrences != 0 {
            break;
        }
        pattern = transform(&pattern);
        occurrences = image.count_occurrences(&pattern);
    }

    // Assuming there are no overlapping occurrences
    Ok(image.count_on_pixels() - occurrences * pattern.count_on_pixels())
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Position {
    x_coord: u32,
    y_coord: u32,
}

impl Position {
    fn new(x_coord: u32, y_coord: u32) -> Self {
        Self { x_coord, y_coord }
    }
}

enum Rotation {
    Nil,
    Left,
    Right,
    Opposite,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, target_dir: Direction) -> Rotation {
        match (self, target_dir) {
            (Self::North, Self::West) => Rotation::Left,
            (Self::West, Self::South) => Rotation::Left,
            (Self::South, Self::East) => Rotation::Left,
            (Self::East, Self::North) => Rotation::Left,
            (Self::North, Self::East) => Rotation::Right,
            (Self::East, Self::South) => Rotation::Right,
            (Self::South, Self::West) => Rotation::Right,
            (Self::West, Self::North) => Rotation::Right,
            (Self::North, Self::South) => Rotation::Opposite,
            (Self::South, Self::North) => Rotation::Opposite,
            (Self::East, Self::West) => Rotation::Opposite,
            (Self::West, Self::East) => Rotation::Opposite,
            _ => Rotation::Nil,
        }
    }
}
