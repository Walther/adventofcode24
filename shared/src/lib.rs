pub struct Maze {
    maze: HashMap<(usize, usize), char>,
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = HashMap::new();
        for (row, contents) in s.lines().enumerate() {
            for (column, character) in contents.chars().enumerate() {
                maze.insert((row, column), character);
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
    maze: &'a Maze,
    x: usize,
    y: usize,
}

impl<'a> Visitor<'a> {
    #[must_use]
    pub fn new(maze: &'a Maze, x: usize, y: usize) -> Self {
        Self { maze, x, y }
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
        self.get()
    }

    pub fn collect(&mut self, max_length: usize, direction: Direction) -> Option<String> {
        let mut collection = String::new();
        let grab = self.get()?;
        collection.push(*grab);

        while collection.len() < max_length {
            match self.step(direction) {
                Some(c) => collection.push(*c),
                None => return Some(collection),
            }
        }

        Some(collection)
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
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
