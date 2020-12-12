use super::*;

pub fn part2(actions: &[Action]) -> i32 {
    let mut ship = Ship::default();
    for action in actions {
        ship.take(action);
    }
    ship.distance_to_origin()
}

struct Ship {
    position: Coordinates,
    waypoint: Coordinates,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Coordinates::default(),
            waypoint: Coordinates::new(1, 10),
        }
    }
}

impl Ship {
    fn take(&mut self, action: &Action) {
        match *action {
            Action::Direction(direction, distance) => {
                self.waypoint.move_direction(direction, distance);
            }
            Action::Forward(times) => self.move_forward(times),
            Action::Left(degrees) => self.waypoint.rotate(-degrees),
            Action::Right(degrees) => self.waypoint.rotate(degrees),
        }
    }

    fn move_forward(&mut self, times: i32) {
        let vertical_dist = times * self.waypoint.vertical;
        let horizontal_dist = times * self.waypoint.horizontal;
        self.position.move_distance(vertical_dist, horizontal_dist);
    }

    fn distance_to_origin(&self) -> i32 {
        self.position.distance_to_origin()
    }
}
