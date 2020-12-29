use super::*;

use std::collections::HashSet;

pub fn part2(seating_area: &SeatingArea) -> usize {
    let mut seat_map = SeatMap::build(seating_area);
    let mut seen = HashSet::new();

    while seen.insert(seat_map.get_occupied_positions()) {
        seat_map.update();
    }

    seat_map.count_occupied()
}

pub struct SeatMap {
    seats: HashMap<Position, Area>,
    near: HashMap<Position, Vec<Position>>,
}

impl SeatMap {
    fn build(seating_area: &SeatingArea) -> Self {
        let seats = seating_area
            .grid
            .iter()
            .filter(|&(_, area)| *area != Area::Floor)
            .map(|(pos, area)| (*pos, *area))
            .collect::<HashMap<_, _>>();

        let near = seats
            .keys()
            .map(|pos| {
                let near_seats = all_directions()
                    .filter_map(|(row_delta, col_delta)| {
                        (1_i32..)
                            .map(|dist| {
                                let near_pos = Position::new(
                                    pos.row + dist * row_delta,
                                    pos.col + dist * col_delta,
                                );
                                (near_pos, seating_area.grid.get(&near_pos))
                            })
                            .take_while(|(_, area)| area.is_some())
                            .find(|(_, area)| *area != Some(&Area::Floor))
                            .map(|(near_pos, _)| near_pos)
                    })
                    .collect::<Vec<_>>();
                (*pos, near_seats)
            })
            .collect::<HashMap<_, _>>();

        Self { seats, near }
    }

    fn get_occupied_positions(&self) -> BTreeSet<Position> {
        self.seats
            .iter()
            .filter(|&(_, area)| *area == Area::Occupied)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|&area| *area == Area::Occupied)
            .count()
    }

    fn near_seats(&self, pos: &Position) -> impl Iterator<Item = &Position> {
        self.near.get(pos).unwrap().iter()
    }

    fn update(&mut self) {
        let updated_seats = self
            .seats
            .keys()
            .map(|pos| (*pos, self.update_position(pos)))
            .collect();
        self.seats = updated_seats;
    }

    fn update_position(&self, pos: &Position) -> Area {
        match self.seats.get(pos) {
            Some(Area::Empty) => {
                if self
                    .near_seats(pos)
                    .any(|adj| self.seats.get(adj) == Some(&Area::Occupied))
                {
                    Area::Empty
                } else {
                    Area::Occupied
                }
            }
            Some(Area::Occupied) => {
                if self
                    .near_seats(pos)
                    .filter(|adj| self.seats.get(adj) == Some(&Area::Occupied))
                    .count()
                    >= 5
                {
                    Area::Empty
                } else {
                    Area::Occupied
                }
            }
            _ => panic!("Tried to update position with no seat."),
        }
    }
}
