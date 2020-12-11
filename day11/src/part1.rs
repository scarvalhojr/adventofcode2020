use super::*;

use std::collections::HashSet;

pub fn part1(seating_area: &SeatingArea) -> usize {
    let mut seat_map = SeatMap::build(seating_area);
    let mut seen = HashSet::new();

    while seen.insert(seat_map.get_occupied_positions()) {
        seat_map.update();
    }

    seat_map.count_occupied()
}

pub struct SeatMap {
    seats: HashMap<Position, Area>,
    adjacent: HashMap<Position, Vec<Position>>,
}

impl SeatMap {
    fn build(seating_area: &SeatingArea) -> Self {
        let seats = seating_area
            .grid
            .iter()
            .filter(|&(_, area)| *area != Area::Floor)
            .map(|(pos, area)| (*pos, *area))
            .collect::<HashMap<_, _>>();

        let adjacent = seats
            .keys()
            .map(|pos| {
                let adjacent_seats = pos
                    .adjacents()
                    .filter(|pos| seats.contains_key(pos))
                    .collect::<Vec<_>>();
                (*pos, adjacent_seats)
            })
            .collect::<HashMap<_, _>>();

        Self { seats, adjacent }
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

    fn adjacent_seats(
        &self,
        pos: &Position,
    ) -> impl Iterator<Item = &Position> {
        self.adjacent.get(pos).unwrap().iter()
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
                    .adjacent_seats(pos)
                    .any(|adj| self.seats.get(adj) == Some(&Area::Occupied))
                {
                    Area::Empty
                } else {
                    Area::Occupied
                }
            }
            Some(Area::Occupied) => {
                if self
                    .adjacent_seats(pos)
                    .filter(|adj| self.seats.get(adj) == Some(&Area::Occupied))
                    .count()
                    >= 4
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
