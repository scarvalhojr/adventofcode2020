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

// fn update_position_v2(&self, pos: &Position) -> Area {
//     // TODO: avoid unwrap!
//     match self.grid.get(pos).unwrap() {
//         Area::Empty if self.count_occupied_dir(pos) == 0 => {
//             Area::Occupied
//         }
//         Area::Occupied if self.count_occupied_dir(pos) >= 5 => {
//             Area::Empty
//         }
//         area => {
//             area.clone()
//         }
//     }
// }

// fn closest_dir(&self, pos: &Position, delta_x: i32, delta_y: i32) -> Option<Area> {
//     (1_i32..)
//         .map(|dist| self.grid.get(&Position::new(pos.row + dist * delta_x, pos.col + dist * delta_y)))
//         .take_while(|a| a.is_some())
//         .map(|a| *a.unwrap())
//         .find(|a| *a != Area::Floor)
// }

// fn count_occupied_dir(&self, pos: &Position) -> usize {
//     (-1_i32..=1_i32)
//         .flat_map(|y_diff: i32| {
//             (-1_i32..=1_i32)
//                 .map(move |x_diff: i32| (y_diff, x_diff))
//         })
//         .filter(|(x_diff, y_diff)| {
//             *x_diff != 0 || *y_diff != 0
//         })
//         .map(|(x_diff, y_diff)| self.closest_dir(pos, x_diff, y_diff))
//         .filter(|a | *a == Some(Area::Occupied))
//         .count()
// }
