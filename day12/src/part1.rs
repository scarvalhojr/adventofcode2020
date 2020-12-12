use super::*;

pub fn part1(actions: &[Action]) -> i32 {
    let mut ship = Ship::default();
    for action in actions {
        ship.take(action);
    }
    ship.distance_to_origin()
}

struct Ship {
    position: Coordinates,
    direction: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Coordinates::default(),
            direction: Direction::East,
        }
    }
}

impl Ship {
    fn take(&mut self, action: &Action) {
        match *action {
            Action::Direction(direction, distance) => {
                self.position.move_direction(direction, distance);
            }
            Action::Forward(distance) => {
                self.position.move_direction(self.direction, distance);
            }
            Action::Left(degrees) => {
                self.direction = self.direction.turn(-degrees).unwrap()
            }
            Action::Right(degrees) => {
                self.direction = self.direction.turn(degrees).unwrap()
            }
        }
    }

    fn distance_to_origin(&self) -> i32 {
        self.position.distance_to_origin()
    }
}
