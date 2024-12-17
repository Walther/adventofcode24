use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use nalgebra::{Point2, Vector2};

#[rustfmt::skip]
use Direction::{NW, N, NE, W, E, SW, S, SE};

pub type Coordinate = Point2<isize>;

pub type Displacement = Vector2<isize>;

#[derive(Clone, Default)]
pub struct Maze {
    maze: HashMap<Point2<isize>, char>,
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = HashMap::new();
        for (row, contents) in s.lines().enumerate() {
            for (column, character) in contents.chars().enumerate() {
                // FIXME: ugly casts
                #[allow(clippy::cast_possible_wrap)]
                let coordinate = Coordinate::new(
                    column.try_into().expect("Number conversion error"),
                    row.try_into().expect("Number conversion error"),
                );
                maze.insert(coordinate, character);
            }
        }

        Ok(Maze { maze })
    }
}

impl Maze {
    #[must_use]
    pub fn all_coordinates(&self) -> Vec<&Coordinate> {
        self.maze.keys().collect()
    }

    #[must_use]
    pub fn all_values(&self) -> Vec<char> {
        self.maze.values().copied().collect()
    }

    #[must_use]
    pub fn find(&self, search: char) -> Option<Coordinate> {
        self.maze
            .iter()
            .find(|(_, &character)| character == search)
            .map(|(&coordinate, _)| coordinate)
    }

    #[must_use]
    pub fn find_all(&self, search: char) -> Vec<Coordinate> {
        self.maze
            .iter()
            .filter(|(_, &character)| character == search)
            .map(|(&coordinate, _)| coordinate)
            .collect()
    }

    #[must_use]
    pub fn get(&self, coordinate: Coordinate) -> Option<&char> {
        self.maze.get(&coordinate)
    }

    pub fn upsert(&mut self, coordinate: Coordinate, v: char) -> Option<char> {
        self.maze.insert(coordinate, v)
    }

    #[must_use]
    pub fn contains_coordinate(&self, coordinate: Coordinate) -> bool {
        self.maze.contains_key(&coordinate)
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

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [NW, N, NE, W, E, SW, S, SE].iter().copied()
    }
}

#[derive(Clone)]
pub struct Visitor<'a> {
    options: VisitorOptions,
    maze: &'a Maze,
    coordinate: Coordinate,
    path: Vec<(Coordinate, Direction)>,
    visited: HashSet<(Coordinate, Direction)>,
    has_looped: bool,
    pockets: Vec<char>,
}

#[derive(Default, Clone)]
pub struct VisitorOptions {
    pub record_visited: bool,
    pub has_pockets: bool,
}

impl<'a> Visitor<'a> {
    #[must_use]
    pub fn new(options: VisitorOptions, maze: &'a Maze, coordinate: Coordinate) -> Self {
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        let has_looped = false;
        if options.record_visited {
            path.push((coordinate, N));
            visited.insert((coordinate, N));
        };
        let pockets = Vec::new();
        Self {
            options,
            maze,
            coordinate,
            path,
            visited,
            has_looped,
            pockets,
        }
    }

    #[must_use]
    pub fn position(&self) -> Coordinate {
        self.coordinate
    }

    #[must_use]
    pub fn get(&self) -> Option<&char> {
        self.maze.get(self.coordinate)
    }

    #[must_use]
    pub fn coordinate_in_direction(&self, direction: Direction) -> Option<Coordinate> {
        let x;
        let y;
        match direction {
            Direction::NW => {
                x = self.coordinate.x.checked_sub(1)?;
                y = self.coordinate.y.checked_sub(1)?;
            }
            Direction::N => {
                x = self.coordinate.x;
                y = self.coordinate.y.checked_sub(1)?;
            }
            Direction::NE => {
                x = self.coordinate.x.checked_add(1)?;
                y = self.coordinate.y.checked_sub(1)?;
            }
            Direction::W => {
                x = self.coordinate.x.checked_sub(1)?;
                y = self.coordinate.y;
            }
            Direction::E => {
                x = self.coordinate.x.checked_add(1)?;
                y = self.coordinate.y;
            }
            Direction::SW => {
                x = self.coordinate.x.checked_sub(1)?;
                y = self.coordinate.y.checked_add(1)?;
            }
            Direction::S => {
                x = self.coordinate.x;
                y = self.coordinate.y.checked_add(1)?;
            }
            Direction::SE => {
                x = self.coordinate.x.checked_add(1)?;
                y = self.coordinate.y.checked_add(1)?;
            }
        }
        let coordinate = Coordinate::new(x, y);
        Some(coordinate)
    }

    #[must_use]
    pub fn peek(&self, direction: Direction) -> Option<&char> {
        let coordinate = self.coordinate_in_direction(direction)?;
        self.maze.get(coordinate)
    }

    pub fn step(&mut self, direction: Direction) -> Option<&char> {
        let coordinate = self.coordinate_in_direction(direction)?;
        self.coordinate.x = coordinate.x;
        self.coordinate.y = coordinate.y;
        if self.options.record_visited {
            self.path.push((coordinate, direction));
            let unique = self.visited.insert((coordinate, direction));
            if !unique {
                self.has_looped = true;
            }
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
    pub fn surroundings(&self) -> [Option<&char>; 9] {
        [
            self.peek(NW),
            self.peek(N),
            self.peek(NE),
            self.peek(W),
            self.get(),
            self.peek(E),
            self.peek(SW),
            self.peek(S),
            self.peek(SE),
        ]
    }

    #[must_use]
    pub fn surroundings_nwes(&self) -> [Option<&char>; 4] {
        [self.peek(N), self.peek(W), self.peek(E), self.peek(S)]
    }

    #[must_use]
    pub fn path(&self) -> Option<&Vec<(Coordinate, Direction)>> {
        match self.options.record_visited {
            true => Some(&self.path),
            false => None,
        }
    }

    #[must_use]
    pub fn visited_coordinates(&self) -> Vec<Coordinate> {
        self.visited
            .iter()
            .map(|(coordinate, _direction)| *coordinate)
            .unique()
            .collect()
    }

    #[must_use]
    pub fn has_looped(&self) -> bool {
        self.has_looped
    }

    /// Returns the flood fill coordinates from this [`Visitor`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the color for the fill cannot be determined.
    pub fn flood_nwes(&self) -> Result<HashSet<Coordinate>, String> {
        let color = self.get().ok_or("Unable to choose visitor color")?;
        let mut coordinates = HashSet::new();
        coordinates.insert(self.position());
        loop {
            let before = coordinates.len();
            for coordinate in coordinates.clone() {
                let visitor = Visitor::new(VisitorOptions::default(), self.maze, coordinate);
                for neighbor in [N, W, E, S]
                    .iter()
                    .filter_map(|&d| visitor.coordinate_in_direction(d))
                {
                    if !coordinates.contains(&neighbor) && self.maze.get(neighbor) == Some(color) {
                        coordinates.insert(neighbor);
                    }
                }
            }
            let after = coordinates.len();
            if before == after {
                break;
            }
        }

        Ok(coordinates)
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    const NUMPAD_MAZE_STR: &str = "123\n456\n789";

    #[test]
    fn surroundings() {
        let maze: Maze = NUMPAD_MAZE_STR.parse().expect("Unable to parse maze");
        let start: Coordinate = Coordinate::new(1, 1);
        let visitor = Visitor::new(VisitorOptions::default(), &maze, start);
        let surroundings = visitor.surroundings();
        let expected = [
            Some(&'1'),
            Some(&'2'),
            Some(&'3'),
            Some(&'4'),
            Some(&'5'),
            Some(&'6'),
            Some(&'7'),
            Some(&'8'),
            Some(&'9'),
        ];
        assert_eq!(surroundings, expected);
    }
}
