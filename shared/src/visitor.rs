#![allow(clippy::missing_panics_doc)] // TODO: fix various .expects()

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use itertools::Itertools;

use crate::{
    Coordinate,
    Direction::{self, E, N, NE, NW, S, SE, SW, W},
    Maze,
};

#[derive(Clone)]
pub struct Visitor {
    maze: Arc<Mutex<Maze>>,
    coordinate: Coordinate,
    path: Vec<(Coordinate, Direction)>,
    visited: HashSet<(Coordinate, Direction)>,
    has_looped: bool,
    pockets: Vec<char>,
}

impl Visitor {
    #[must_use]
    pub fn new(maze: &Arc<Mutex<Maze>>, coordinate: Coordinate) -> Self {
        let maze = maze.clone();
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        let has_looped = false;
        path.push((coordinate, N));
        visited.insert((coordinate, N));
        let pockets = Vec::new();
        Self {
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
    pub fn get(&self) -> Option<char> {
        self.maze
            .lock()
            .expect("Failed to acquire lock")
            .get(self.coordinate)
    }

    pub fn sudo_upsert(&mut self, coordinate: Coordinate, character: char) {
        self.maze
            .lock()
            .expect("Failed to acquire lock")
            .upsert(coordinate, character);
    }

    #[must_use]
    pub fn get_maze(&self) -> Arc<Mutex<Maze>> {
        self.maze.clone()
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
    pub fn peek(&self, direction: Direction) -> Option<char> {
        let coordinate = self.coordinate_in_direction(direction)?;
        self.maze
            .lock()
            .expect("Failed to acquire lock")
            .get(coordinate)
    }

    #[must_use]
    pub fn distance_to(&self, search: char, direction: Direction) -> Option<usize> {
        let mut ghost = self.clone();
        let mut distance = 0;
        while let Some(c) = ghost.step(direction) {
            distance += 1;
            if c == search {
                return Some(distance);
            }
        }
        None
    }

    #[must_use]
    pub fn coordinate_in_distance(
        &self,
        distance: usize,
        direction: Direction,
    ) -> Option<Coordinate> {
        let mut ghost = self.clone();
        for _ in 0..distance {
            ghost.step(direction)?;
        }
        Some(ghost.position())
    }

    pub fn step(&mut self, direction: Direction) -> Option<char> {
        let coordinate = self.coordinate_in_direction(direction)?;
        self.coordinate.x = coordinate.x;
        self.coordinate.y = coordinate.y;
        self.path.push((coordinate, direction));

        let unique = self.visited.insert((coordinate, direction));
        if !unique {
            self.has_looped = true;
        }

        self.get()
    }

    pub fn collect(&mut self, max_length: usize, direction: Direction) -> Option<&Vec<char>> {
        while self.pockets.len() < max_length {
            let grab = self.get()?;
            self.pockets.push(grab);
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
    pub fn surroundings(&self) -> [Option<char>; 9] {
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
    pub fn surroundings_nwes(&self) -> [Option<char>; 4] {
        [self.peek(N), self.peek(W), self.peek(E), self.peek(S)]
    }

    #[must_use]
    pub fn coordinates_nwes(&self) -> Vec<Coordinate> {
        [
            self.coordinate_in_direction(N),
            self.coordinate_in_direction(W),
            self.coordinate_in_direction(E),
            self.coordinate_in_direction(S),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[must_use]
    pub fn path(&self) -> &Vec<(Coordinate, Direction)> {
        &self.path
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
                let visitor = Visitor::new(&self.maze.clone(), coordinate);
                for neighbor in [N, W, E, S]
                    .iter()
                    .filter_map(|&d| visitor.coordinate_in_direction(d))
                {
                    if !coordinates.contains(&neighbor)
                        && self
                            .maze
                            .lock()
                            .expect("Unable to acquire lock")
                            .get(neighbor)
                            == Some(color)
                    {
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
pub(crate) mod unit {
    use super::*;
    const NUMPAD_MAZE_STR: &str = "123\n456\n789";

    #[test]
    fn surroundings() {
        let maze: Arc<Mutex<Maze>> = NUMPAD_MAZE_STR
            .parse::<Maze>()
            .expect("Unable to parse maze")
            .make_shareable();
        let start: Coordinate = Coordinate::new(1, 1);
        let visitor = Visitor::new(&maze, start);
        let surroundings = visitor.surroundings();
        let expected = [
            Some('1'),
            Some('2'),
            Some('3'),
            Some('4'),
            Some('5'),
            Some('6'),
            Some('7'),
            Some('8'),
            Some('9'),
        ];
        assert_eq!(surroundings, expected);
    }

    #[test]
    fn coordinates_nwes() {
        let maze: Arc<Mutex<Maze>> = NUMPAD_MAZE_STR
            .parse::<Maze>()
            .expect("Unable to parse maze")
            .make_shareable();
        let start: Coordinate = Coordinate::new(1, 1);
        let visitor = Visitor::new(&maze, start);
        let coordinates_nwes = visitor.coordinates_nwes();
        let expected = [
            Coordinate::new(1, 0),
            Coordinate::new(0, 1),
            Coordinate::new(2, 1),
            Coordinate::new(1, 2),
        ];
        assert_eq!(coordinates_nwes, expected);
    }
}
