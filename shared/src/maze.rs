use std::{collections::HashMap, str::FromStr};

use nalgebra::Point2;

use crate::Coordinate;

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
    pub fn find_replace(&mut self, search: char, replace: char) -> Option<Coordinate> {
        let (&coordinate, _) = self.maze.iter().find(|&(_coord, ch)| *ch == search)?;
        self.upsert(coordinate, replace)?;
        Some(coordinate)
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
