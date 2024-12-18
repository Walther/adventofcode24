use itertools::Itertools;
use shared::{Coordinate, Displacement, Maze};

#[cfg(test)]
pub(crate) const BATHROOM_WIDTH: isize = 11;
#[cfg(test)]
pub(crate) const BATHROOM_HEIGHT: isize = 7;
#[cfg(not(test))]
pub(crate) const BATHROOM_WIDTH: isize = 101;
#[cfg(not(test))]
pub(crate) const BATHROOM_HEIGHT: isize = 103;

#[derive(Clone)]
pub struct Robot {
    position: Coordinate,
    velocity: Displacement,
}

impl Robot {
    pub fn step(&mut self) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(BATHROOM_WIDTH);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(BATHROOM_HEIGHT);
    }

    pub fn step_n(&mut self, n: isize) {
        self.position.x = (self.position.x + (n * self.velocity.x)).rem_euclid(BATHROOM_WIDTH);
        self.position.y = (self.position.y + (n * self.velocity.y)).rem_euclid(BATHROOM_HEIGHT);
    }
}

impl TryFrom<&str> for Robot {
    type Error = String;

    // FIXME: sometimes ? can be really nice, sometimes stuff mixing options and results becomes a pain
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (position, velocity) = line.split_once(' ').expect("Robot parse error");
        let (_tag, position) = position.split_once("p=").expect("Robot parse error");
        let (x, y) = position.split_once(',').expect("Robot parse error");
        let x: isize = x.parse().expect("Robot parse error");
        let y: isize = y.parse().expect("Robot parse error");

        let (_tag, velocity) = velocity.split_once("v=").expect("Robot parse error");
        let (dx, dy) = velocity.split_once(',').expect("Robot parse error");
        let dx: isize = dx.parse().expect("Robot parse error");
        let dy: isize = dy.parse().expect("Robot parse error");

        let position = Coordinate::new(x, y);
        let velocity = Displacement::new(dx, dy);

        Ok(Robot { position, velocity })
    }
}

#[must_use]
pub fn safety_score(robots: &Vec<Robot>) -> usize {
    let x_axis = BATHROOM_WIDTH / 2;
    let y_axis = BATHROOM_HEIGHT / 2;
    let mut quads: [usize; 4] = [0, 0, 0, 0];

    for robot in robots {
        let x = robot.position.x;
        let y = robot.position.y;
        match (x.cmp(&x_axis), y.cmp(&y_axis)) {
            // 01
            // 23
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quads[0] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quads[1] += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quads[2] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quads[3] += 1,
            _ => continue,
        }
    }

    quads.iter().product()
}

#[must_use]
pub fn looks_like_a_tree(robots: &[Robot]) -> bool {
    robots.iter().map(|r| r.position).all_unique()
}

pub fn print_bathroom(robots: &[Robot]) {
    let mut bathroom = Maze::default();
    for robot in robots {
        bathroom.upsert(robot.position, 'R');
    }
    for y in 0..BATHROOM_HEIGHT {
        for x in 0..BATHROOM_WIDTH {
            match bathroom.get(Coordinate::new(x, y)) {
                Some(r) => print!("{r}"),
                None => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod unit {
    use crate::robot::{safety_score, Robot};
    use shared::{Coordinate, Displacement};

    #[test]
    fn quad_4() {
        let robot_0 = Robot {
            position: Coordinate::new(0, 0),
            velocity: Displacement::new(0, 0),
        };
        let robot_1 = Robot {
            position: Coordinate::new(6, 0),
            velocity: Displacement::new(0, 0),
        };
        let robot_2 = Robot {
            position: Coordinate::new(0, 4),
            velocity: Displacement::new(0, 0),
        };
        let robot_3 = Robot {
            position: Coordinate::new(6, 4),
            velocity: Displacement::new(0, 0),
        };
        let robots = vec![robot_0, robot_1, robot_2, robot_3];
        let value = safety_score(&robots);

        let expected = 1;
        assert_eq!(value, expected);
    }
}
