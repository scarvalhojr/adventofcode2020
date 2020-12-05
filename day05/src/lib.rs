use std::convert::TryFrom;
use std::str::FromStr;

const ROW_SPLITS: usize = 7;
const COL_SPLITS: usize = 3;

pub struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

pub fn part1(seats: &[Seat]) -> Option<u32> {
    seats.iter().map(|seat| seat.id()).max()
}

pub fn part2(seats: &[Seat]) -> Option<u32> {
    let mut ids: Vec<_> = seats.iter().map(|seat| seat.id()).collect();

    ids.sort_unstable();
    ids.windows(2)
        .flat_map(<&[u32; 2]>::try_from)
        .find(|&[prev_seat, next_seat]| prev_seat + 1 < *next_seat)
        .map(|[prev_seat, _]| prev_seat + 1)
}

fn parse_bin_str(s: &str, low_ch: char, high_ch: char) -> Option<u32> {
    let power = u32::try_from(s.len()).ok()?;
    s.chars()
        .try_fold((0, 2_u32.pow(power)), |(low, high), ch| match ch {
            c if c == low_ch => Some((low, low + (high - low) / 2)),
            c if c == high_ch => Some((low + (high - low) / 2, high)),
            _ => None,
        })
        .map(|(low, _)| low)
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s
            .get(..ROW_SPLITS)
            .and_then(|r| parse_bin_str(r, 'F', 'B'))
            .ok_or_else(|| String::from("Invalid row"))?;
        let col = s
            .get(ROW_SPLITS..ROW_SPLITS + COL_SPLITS)
            .and_then(|c| parse_bin_str(c, 'L', 'R'))
            .ok_or_else(|| String::from("Invalid column"))?;
        Ok(Seat { row, col })
    }
}
