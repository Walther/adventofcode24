use itertools::Itertools;

#[derive(Clone)]
pub struct Maze {
    maze: HashMap<(usize, usize), char>,
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = HashMap::new();
        for (row, contents) in s.lines().enumerate() {
            for (column, character) in contents.chars().enumerate() {
                maze.insert((column, row), character);
            }
        }

        Ok(Maze { maze })
    }
}

impl Maze {
    #[must_use]
    pub fn all_coordinates(&self) -> Vec<&(usize, usize)> {
        self.maze.keys().collect()
    }

    #[must_use]
    pub fn find(&self, search: char) -> Option<(usize, usize)> {
        self.maze
            .iter()
            .find(|&((_, _), &character)| character == search)
            .map(|((x, y), _)| (*x, *y))
    }

    #[must_use]
    pub fn find_all(&self, search: char) -> Vec<(usize, usize)> {
        self.maze
            .iter()
            .filter(|&((_, _), character)| character == &search)
            .map(|((x, y), _)| (*x, *y))
            .collect()
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.maze.get(&(x, y))
    }

    pub fn upsert(&mut self, x: usize, y: usize, v: char) -> Option<char> {
        self.maze.insert((x, y), v)
    }
}

#[derive(Clone, Copy)]
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
    x: usize,
    y: usize,
    visited: Vec<(usize, usize)>,
    pockets: Vec<char>,
}

#[derive(Default)]
pub struct VisitorOptions {
    pub record_visited: bool,
    pub has_pockets: bool,
}

impl<'a> Visitor<'a> {
    #[must_use]
    pub fn new(options: VisitorOptions, maze: &'a Maze, x: usize, y: usize) -> Self {
        let visited = match options.record_visited {
            true => vec![(x, y)],
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
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    #[must_use]
    pub fn get(&self) -> Option<&char> {
        self.maze.get(self.x, self.y)
    }

    #[must_use]
    pub fn coordinate_in_direction(&self, direction: Direction) -> Option<(usize, usize)> {
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
            self.visited.push((x, y));
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
    pub fn path(&self) -> Option<&Vec<(usize, usize)>> {
        match self.options.record_visited {
            true => Some(&self.visited),
            false => None,
        }
    }

    #[must_use]
    pub fn visited(&self) -> Option<Vec<(usize, usize)>> {
        self.path()
            .map(|path| path.iter().unique().copied().collect())
    }
}

#[cfg(test)]
mod tests {
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
