use itertools::Itertools;

#[derive(Clone)]
pub struct Maze {
    maze: HashMap<(isize, isize), char>,
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = HashMap::new();
        for (row, contents) in s.lines().enumerate() {
            for (column, character) in contents.chars().enumerate() {
                // FIXME: ugly casts
                #[allow(clippy::cast_possible_wrap)]
                maze.insert((column as isize, row as isize), character);
            }
        }

        Ok(Maze { maze })
    }
}

impl Maze {
    #[must_use]
    pub fn all_coordinates(&self) -> Vec<&(isize, isize)> {
        self.maze.keys().collect()
    }

    #[must_use]
    pub fn all_values(&self) -> Vec<char> {
        self.maze.values().copied().collect()
    }

    #[must_use]
    pub fn find(&self, search: char) -> Option<(isize, isize)> {
        self.maze
            .iter()
            .find(|&((_, _), &character)| character == search)
            .map(|((x, y), _)| (*x, *y))
    }

    #[must_use]
    pub fn find_all(&self, search: char) -> Vec<(isize, isize)> {
        self.maze
            .iter()
            .filter(|&((_, _), character)| character == &search)
            .map(|((x, y), _)| (*x, *y))
            .collect()
    }

    #[must_use]
    pub fn get(&self, x: isize, y: isize) -> Option<&char> {
        self.maze.get(&(x, y))
    }

    pub fn upsert(&mut self, x: isize, y: isize, v: char) -> Option<char> {
        self.maze.insert((x, y), v)
    }

    #[must_use]
    pub fn contains_coordinate(&self, x: isize, y: isize) -> bool {
        self.maze.contains_key(&(x, y))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}
use std::{collections::HashMap, str::FromStr};

#[rustfmt::skip]
use Direction::{NW, N, NE, W, E, SW, S, SE};

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [NW, N, NE, W, E, SW, S, SE].iter().copied()
    }
}

pub struct Visitor<'a> {
    options: VisitorOptions,
    maze: &'a Maze,
    x: isize,
    y: isize,
    visited: Vec<((isize, isize), Direction)>,
    pockets: Vec<char>,
}

#[derive(Default)]
pub struct VisitorOptions {
    pub record_visited: bool,
    pub has_pockets: bool,
}

impl<'a> Visitor<'a> {
    #[must_use]
    pub fn new(options: VisitorOptions, maze: &'a Maze, x: isize, y: isize) -> Self {
        let visited = match options.record_visited {
            true => vec![((x, y), N)],
            false => Vec::new(),
        };
        let pockets = Vec::new();
        Self {
            options,
            maze,
            x,
            y,
            visited,
            pockets,
        }
    }

    #[must_use]
    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    #[must_use]
    pub fn get(&self) -> Option<&char> {
        self.maze.get(self.x, self.y)
    }

    #[must_use]
    pub fn coordinate_in_direction(&self, direction: Direction) -> Option<(isize, isize)> {
        let x;
        let y;
        match direction {
            Direction::NW => {
                x = self.x.checked_sub(1)?;
                y = self.y.checked_sub(1)?;
            }
            Direction::N => {
                x = self.x;
                y = self.y.checked_sub(1)?;
            }
            Direction::NE => {
                x = self.x.checked_add(1)?;
                y = self.y.checked_sub(1)?;
            }
            Direction::W => {
                x = self.x.checked_sub(1)?;
                y = self.y;
            }
            Direction::E => {
                x = self.x.checked_add(1)?;
                y = self.y;
            }
            Direction::SW => {
                x = self.x.checked_sub(1)?;
                y = self.y.checked_add(1)?;
            }
            Direction::S => {
                x = self.x;
                y = self.y.checked_add(1)?;
            }
            Direction::SE => {
                x = self.x.checked_add(1)?;
                y = self.y.checked_add(1)?;
            }
        }

        Some((x, y))
    }

    #[must_use]
    pub fn peek(&self, direction: Direction) -> Option<&char> {
        let (x, y) = self.coordinate_in_direction(direction)?;
        self.maze.get(x, y)
    }

    pub fn step(&mut self, direction: Direction) -> Option<&char> {
        let (x, y) = self.coordinate_in_direction(direction)?;
        self.x = x;
        self.y = y;
        if self.options.record_visited {
            self.visited.push(((x, y), direction));
        }
        self.get()
    }

    pub fn collect(&mut self, max_length: usize, direction: Direction) -> Option<&Vec<char>> {
        if !self.options.has_pockets {
            return None;
        };

        while self.pockets.len() < max_length {
            let grab = self.get()?;
            self.pockets.push(*grab);
            match self.peek(direction) {
                Some(_) => {
                    self.step(direction);
                }
                None => return Some(&self.pockets),
            }
        }

        Some(&self.pockets)
    }

    #[must_use]
    pub fn surroundings(&self) -> Option<[&char; 9]> {
        Some([
            self.peek(NW)?,
            self.peek(N)?,
            self.peek(NE)?,
            self.peek(W)?,
            self.get()?,
            self.peek(E)?,
            self.peek(SW)?,
            self.peek(S)?,
            self.peek(SE)?,
        ])
    }

    #[must_use]
    pub fn path(&self) -> Option<&Vec<((isize, isize), Direction)>> {
        match self.options.record_visited {
            true => Some(&self.visited),
            false => None,
        }
    }

    #[must_use]
    pub fn unique_visited(&self) -> Option<Vec<(isize, isize)>> {
        let path = self.path()?;
        Some(
            path.iter()
                .map(|((x, y), _direction)| (*x, *y))
                .unique()
                .collect(),
        )
    }

    /// Checks that all visited location-direction pairs are unique.
    ///
    /// Computationally intense, slow!
    #[must_use]
    pub fn has_looped(&self) -> Option<bool> {
        if !self.options.record_visited {
            return None;
        };
        Some(!self.visited.iter().all_unique())
    }

    /// Checks if the upcoming location-direction pair already exists in the visited list.
    ///
    /// Computationally faster than `has_looped`, but only checks the upcoming step.
    #[must_use]
    pub fn deja_vu(&self, direction: Direction) -> bool {
        let Some(next) = self.coordinate_in_direction(direction) else {
            return false;
        };
        self.visited.contains(&(next, direction))
    }
}

#[cfg(test)]
mod maze {
    use super::*;
    const NUMPAD_MAZE_STR: &str = "123\n456\n789";

    #[test]
    fn surroundings() {
        let maze: Maze = NUMPAD_MAZE_STR.parse().expect("Unable to parse maze");
        let visitor = Visitor::new(VisitorOptions::default(), &maze, 1, 1);
        let surroundings = visitor.surroundings().expect("No surroundings found");
        let expected = [&'1', &'2', &'3', &'4', &'5', &'6', &'7', &'8', &'9'];
        assert_eq!(surroundings, expected);
    }
}
