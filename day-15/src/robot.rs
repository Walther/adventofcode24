use shared::{Direction, Visitor};

pub struct Robot {
    visitor: Visitor,
}

impl Robot {
    #[must_use]
    pub fn new(visitor: Visitor) -> Self {
        Robot { visitor }
    }

    /// Attempts to move the robot to the specified direction.
    ///
    /// The robot will not move if it runs into a wall `#`.
    ///
    /// The robot will push movable objects `O` if there is free space in the direction of the push. This may chain with multiple `O` objects. If there is no free space in the direction of the push, the robot will not move.
    ///
    /// # Panics
    ///
    /// Panics if the maze is not well defined. This may occur if the peek returns unknown characters for the robot.
    pub fn r#move(&mut self, direction: Direction) {
        let peek = self.visitor.peek(direction);
        match peek {
            Some('.') => _ = self.visitor.step(direction),
            Some('O') => self.push(direction),
            Some('#') => (),
            _ => panic!("Robot maze not well defined"),
        }
    }

    fn push(&mut self, direction: Direction) {
        let Some(empty_distance) = self.visitor.distance_to('.', direction) else {
            return;
        };
        let Some(wall_distance) = self.visitor.distance_to('#', direction) else {
            return;
        };
        if wall_distance < empty_distance {
            return;
        }
        let Some(empty_coordinate) = self
            .visitor
            .coordinate_in_distance(empty_distance, direction)
        else {
            return;
        };
        let next_coordinate = self
            .visitor
            .coordinate_in_direction(direction)
            .expect("Maze changed after peek");

        self.visitor.sudo_upsert(empty_coordinate, 'O');
        self.visitor.sudo_upsert(next_coordinate, '.');
        self.visitor.step(direction);
    }

    #[must_use]
    pub fn gps_sum(&self) -> usize {
        self.gps_coordinates().iter().sum()
    }

    #[allow(clippy::cast_sign_loss)]
    fn gps_coordinates(&self) -> Vec<usize> {
        self.visitor
            .get_maze()
            .lock()
            .expect("Unable to acquire lock")
            .find_all('O')
            .iter()
            .map(|&coordinate| (100 * coordinate.y + coordinate.x) as usize)
            .collect()
    }
}
